
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


// two functions 
TranslationUnit(
    NonEmpty(
        [FunctionDefinition(FunctionDefinition { 
            prototype: FunctionPrototype { ty: FullySpecifiedType {
                 qualifier: None, ty: TypeSpecifier { ty: Void, array_specifier: None } 
            }, 
                name: Identifier("main"), 
                parameters: [] 
            }, 
            statement: CompoundStatement { 
                statement_list: [] } }
        ), 
        FunctionDefinition(FunctionDefinition { 
            prototype: FunctionPrototype { ty: FullySpecifiedType { 
                qualifier: None, ty: TypeSpecifier { ty: Void, array_specifier: None } 
            }, 
                name: Identifier("main"), 
                parameters: [] 
            }, 
            statement: CompoundStatement { 
                statement_list: [] } })]))

// single if statement
TranslationUnit(NonEmpty([
    FunctionDefinition(
        FunctionDefinition { 
            prototype: FunctionPrototype { ty: FullySpecifiedType { qualifier: None, 
            ty: TypeSpecifier { ty: Void, array_specifier: None } }, name: Identifier("main"),
             parameters: [] }, 
    statement: CompoundStatement { 
        statement_list: [
            Simple(
                Selection(
                    SelectionStatement { 
                        cond: Variable(Identifier("w")), 
                        rest: Statement(
                            Compound(
                                CompoundStatement { 
                                    statement_list: [
                                        Simple(Jump(Return(Some(BoolConst(true)))
        ))] })) }))] } })]))

TranslationUnit(
    NonEmpty([
        Preprocessor(
            Define(
                FunctionLike { ident: Identifier("Bf"), args: [Identifier("p")], value: "p" }))]))

TranslationUnit(
    NonEmpty([
        Preprocessor(
            Define(ObjectLike { ident: Identifier("Bf"), value: "p" }))]))

TranslationUnit(
    NonEmpty(
        [FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType {
                         qualifier: None, 
                         ty: TypeSpecifier { 
                             ty: Void, array_specifier: None 
                         } 
                    }, 
                    name: Identifier("main"), 
                    parameters: [] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [Simple(
                        Expression(
                            Some(
                                Assignment(
                                    Dot(
                                        Variable(Identifier("q")), 
                                        Identifier("xy")
                                    ), 
                                    Equal, 
                                    FunCall(Identifier(Identifier("vec2<f32>")), 
                                        [FloatConst(1.0)])))))] } })]))

    
TranslationUnit(
    NonEmpty([
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, 
                            array_specifier: None } 
                    }, 
                    name: Identifier("main"), 
                    parameters: [] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [
                        Simple(
                            Declaration(
                                InitDeclaratorList(
                                    InitDeclaratorList { 
                                        head: SingleDeclaration { 
                                            ty: FullySpecifiedType { 
                                                qualifier: None, 
                                                ty: TypeSpecifier { 
                                                    ty: TypeName(TypeName("vec2<f32>")), 
                                                    array_specifier: None } 
                                            }, 
                                            name: Some(Identifier("q")), 
                                            array_specifier: None, 
                                            initializer: Some(Simple(FunCall(Identifier(Identifier("vec2<f32>")), 
                                            [Unary(Minus, IntConst(1)), IntConst(3)]))) }, 
                                        tail: [] })))] } })]))

FunctionDefinition(
    FunctionDefinition { 
        prototype: FunctionPrototype { 
            ty: FullySpecifiedType { 
                qualifier: None, 
                ty: TypeSpecifier { 
                    ty: Void, 
                    array_specifier: None } 
            }, 
            name: Identifier("main"), 
            parameters: [] 
        }, 
        statement: CompoundStatement { 
            statement_list: [Simple(
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
                                name: Some(Identifier("q")), 
                                array_specifier: None, 
                                initializer: Some(Simple(Ternary(Variable(Identifier("w")), 
                                    IntConst(1), IntConst(4)))) }, 
                            tail: [] })))] } })]))

(NonEmpty(
    [FunctionDefinition(
        FunctionDefinition { 
            prototype: FunctionPrototype { 
                ty: FullySpecifiedType { 
                    qualifier: None, 
                    ty: TypeSpecifier { 
                        ty: Void, array_specifier: None } 
                }, 
                name: Identifier("mainImage"), 
                parameters: []
            }, 
            statement: CompoundStatement { 
                statement_list: [
                    Simple(Iteration(For(Declaration(InitDeclaratorList
                        (InitDeclaratorList { 
                            head: SingleDeclaration { 
                                ty: FullySpecifiedType { 
                                    qualifier: None, 
                                    ty: TypeSpecifier { 
                                        ty: Float, 
                                        array_specifier: None } 
                                }, 
                                name: Some(Identifier("i")), 
                            array_specifier: None, 
                            initializer: Some(Simple(Variable(Identifier("a")))) 
                        }, tail: [] })), ForRestStatement { condition: Some(Expr(Binary(LT, Variable(Identifier("i")), Variable(Identifier("b"))))), post_expr: Some(Assignment(Variable(Identifier("i")), Equal, Binary(Add, Variable(Identifier("i")), IntConst(1)))) }, Compound(CompoundStatement { statement_list: [] }))))] } })]))

                        Assignment(Variable(Identifier("i")), Equal, Binary(Add, Variable(Identifier("i")), IntConst(1)))

Simple(Iteration(For(Declaration(
    InitDeclaratorList(InitDeclaratorList { 
        head: SingleDeclaration { 
            ty: FullySpecifiedType { 
                qualifier: None, 
                ty: TypeSpecifier { 
                    ty: Float, 
                    array_specifier: None } 
                }, 
            name: Some(Identifier("i")), 
            array_specifier: None, 
            initializer: Some(Simple(IntConst(1))) 
        }, 
        tail: [] })
), 
ForRestStatement { 
    condition: Some(Expr(Binary(LT, Variable(Identifier("i")), IntConst(2)))), 
    post_expr: Some(Assignment(Variable(Identifier("i")), Equal, 
        Binary(Add, Variable(Identifier("i")), IntConst(1)))) 
}, 
Compound(CompoundStatement { statement_list: [] }))))] } })]))

TranslationUnit(
    NonEmpty([
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, array_specifier: None } 
                    }, 
                    name: Identifier("mainImage"), 
                    parameters: [
                        Named(
                            Some(TypeQualifier { qualifiers: NonEmpty([Storage(Out)]) }), 
                            FunctionParameterDeclarator { 
                                ty: TypeSpecifier { 
                                    ty: Vec4, 
                                    array_specifier: None }, 
                                ident: ArrayedIdentifier { 
                                    ident: Identifier("U"), array_spec: None } }
                        ), 
                        Named(
                            Some(TypeQualifier { qualifiers: NonEmpty([Storage(In)]) }), 
                            FunctionParameterDeclarator { 
                                ty: TypeSpecifier { 
                                    ty: TypeName(TypeName("vec2<f32>")), 
                                    array_specifier: None 
                                }, 
                                ident: ArrayedIdentifier { 
                                    ident: Identifier("pos"), 
                                    array_spec: None } 
                            })] 
                }, 
                statement: CompoundStatement { statement_list: [] } })]))

TranslationUnit(
    NonEmpty([
        Preprocessor(
            Define(
                FunctionLike { 
                    ident: Identifier("P"), 
                    args: [Identifier("p")], 
                    value: "texture(iChannel0, p)" })
        ), 
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, 
                            array_specifier: None } 
                    }, 
                    name: Identifier("main"), 
                    parameters: [] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [Simple(Expression(Some(FunCall(
                        Identifier(Identifier("P")), 
                        [Variable(Identifier("ch0"))]
))))] } })]))


TranslationUnit(
    NonEmpty([
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, 
                            array_specifier: None } 
                    }, 
                    name: Identifier("func"), 
                    parameters: [
                        Named(Some(
                            TypeQualifier { 
                                qualifiers: NonEmpty([Storage(InOut)]) }
                            ), 
                            FunctionParameterDeclarator { 
                                ty: TypeSpecifier { 
                                    ty: Float, 
                                    array_specifier: None 
                                }, 
                                ident: ArrayedIdentifier { 
                                    ident: Identifier("P"), 
                                    array_spec: None } })] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [
                        Simple(Expression(
                            Some(Assignment(Dot(Variable(Identifier("P")), 
                            Identifier("x")
                        ), 
                        Add, FloatConst(1.0)))))] } })]))

// var<private, _>
TranslationUnit(NonEmpty([
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
                    name: Some(Identifier("q")), 
                    array_specifier: None, 
                    initializer: None }, 
                tail: [] })), 
        FunctionDefinition(FunctionDefinition { prototype: FunctionPrototype { ty: FullySpecifiedType { qualifier: None, ty: TypeSpecifier { ty: Void, array_specifier: None } }, name: Identifier("main"), parameters: [] }, statement: CompoundStatement { statement_list: [] } })]))


TranslationUnit(
    NonEmpty([
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, 
                            array_specifier: None } 
                    }, 
                    name: Identifier("norm"), 
                    parameters: [Named(None, FunctionParameterDeclarator { 
                        ty: TypeSpecifier { ty: Vec3, array_specifier: None }, 
                        ident: ArrayedIdentifier { ident: Identifier("po"), array_spec: None } 
                    })] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [
                        Simple(Selection(SelectionStatement { cond: Binary(
                            GT, 
                            Dot(Variable(Identifier("r")), Identifier("x")),
                            Dot(Variable(Identifier("d")), Identifier("x"))
                        ), 
                        rest: Statement(Simple(Expression(Some(Assignment(
                            Variable(Identifier("r")), 
                            Equal, 
                            Variable(Identifier("d"))
            ))))) }))] } })]))

TranslationUnit(
    NonEmpty([
        FunctionDefinition(
            FunctionDefinition { 
                prototype: FunctionPrototype { 
                    ty: FullySpecifiedType { 
                        qualifier: None, 
                        ty: TypeSpecifier { 
                            ty: Void, 
                            array_specifier: None } 
                    }, 
                    name: Identifier("norm"), 
                    parameters: [
                        Named(None, FunctionParameterDeclarator { 
                            ty: TypeSpecifier { 
                                ty: Vec3, 
                                array_specifier: None 
                            }, 
                            ident: ArrayedIdentifier { 
                                ident: Identifier("po"), 
                                array_spec: None } })] 
                }, 
                statement: CompoundStatement { 
                    statement_list: [Simple(Selection(SelectionStatement { 
                        cond: Binary(GT, 
                        Dot(
                            Variable(Identifier("r")), 
                            Identifier("x")), 
                        Dot(Variable(Identifier("d")), Identifier("x"))
                        ), 
                        rest: Statement(Compound(
                            CompoundStatement { 
                                statement_list: [
                                    Simple(
                                        Expression(
                                            Some(
                                                Assignment(
                                                    Variable(Identifier("r")), 
                                                    Equal, 
                                                    Variable(Identifier("d")))))
                                    ), 
                                    Simple(Expression(Some(Assignment(Variable(Identifier("t")), Equal, IntConst(3)))))] })) }))] } })]))


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
            name: Identifier("main"), 
            parameters: [] 
        }, 
        statement: CompoundStatement { 
            statement_list: [Simple(Declaration(InitDeclaratorList(InitDeclaratorList { 
                head: SingleDeclaration { 
                    ty: FullySpecifiedType {
                         qualifier: None, 
                         ty: TypeSpecifier { 
                             ty: Float, 
                             array_specifier: None } 
                    }, 
                    name: Some(Identifier("a")), 
                    array_specifier: None, 
                    initializer: Some(Simple(FunCall(Identifier(Identifier("texelFetch")),
                         [
                            Variable(Identifier("ch0")), 
                            FunCall(Identifier(Identifier("Bi")), 
                                [Variable(Identifier("q"))]), 
                            Variable(Identifier("ch0"))]))) }, tail: [] })))] } })]))