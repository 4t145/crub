
pub const FN_QUERY:&str = r#"
(translation_unit 
    (function_definition 
        type: (_) @type
        declarator:(_) @declarator 
        body: (_)))
"#;

pub const STRUCT_TYPE_QUERY:&str = r#"
(type_definition ;
    type: 
        (struct_specifier 
            name: (type_identifier) @struct_type.name
            body: (_))
    declarator: (type_identifier)  @struct_type.declarator )
"#;

pub const CONST_QUERY:&str = r#"
(translation_unit 
    (declaration
        (type_qualifier) @qualifier 
        type: (_) @type
      declarator:(
          (init_declarator 
              declarator:(identifier) @id
              value:(_)))))
"#;