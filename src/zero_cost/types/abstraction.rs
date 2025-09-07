use crate::{
    zc_abstract,
    zero_cost::{BorrowedValueVisitor, types::wrappers::*},
};
use simd_json::prelude::{ValueAsObject, ValueAsScalar};

zc_abstract! {
    pub struct SourceUnit {
        absolute_path => ["absolutePath"]: ZcStr::<'_>,
        exported_symbols => ["exportedSymbols"]: ZcHashMap::<'_, ZcVec::<'_, ZcIsize::<'_>>>,
        id: ZcIsize::<'_>,
        license: ZcOption::<'_, ZcStr::<'_>>,
        nodes: ZcVec::<'_, ZcDirective::<'_>>,
        stc: ZcStr::<'_>,
    }

    pub enum Directive {
        with_value:
        EventDefinition,
        ContractDefinition,
        EnumDefinition,
        ErrorDefinition,
        FunctionDefinition,
        ImportDirective,
        PragmaDirective,
        StructDefinition,
        UserDefinedValueTypeDefinition,
        UsingForDirective,
        VariableDeclaration,
    }

    pub struct EventDefinition {
        anonymous: ZcBool::<'_>,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        event_selector => ["eventSelector"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        parameters: ZcParameterList::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct ContractDefinition {
        _abstract => ["abstract"]: ZcOption::<'_, ZcBool::<'_>>,
        base_contracts => ["baseContracts"]: ZcVec::<'_, ZcInheritanceSpecifier::<'_>>,
        canonical_name => ["canonicalName"]: ZcOption::<'_, ZcStr::<'_>>,
        contract_dependencies => ["contractDependencies"]: ZcVec::<'_, ZcIsize::<'_>>,
        contract_kind => ["contractKind"]: ZcContractKind,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        fully_implemented => ["fullyImplemented"]: ZcBool::<'_>,
        id: ZcIsize::<'_>,
        linearized_base_contracts => ["linearizedBaseContracts"]: ZcVec::<'_, ZcIsize::<'_>>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        nodes: ZcVec::<'_, ZcBaseNode::<'_>>,
        scope: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        used_errors => ["usedErrors"]: ZcOption::<'_, ZcVec::<'_, ZcIsize::<'_>>>,
    }

    pub struct EnumDefinition {
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        canonical_name => ["canonicalName"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        members: ZcVec::<'_, ZcEnumValue::<'_>>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct EnumValue {
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct ErrorDefinition {
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        error_selector => ["errorSelector"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        parameters: ZcParameterList::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct FunctionDefinition {
        base_functions => ["baseFunctions"]: ZcOption::<'_, ZcVec::<'_, ZcIsize::<'_>>>,
        body: ZcOption::<'_, ZcBlock::<'_>>,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        function_selector => ["functionSelector"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        implemented: ZcBool::<'_>,
        kind: ZcFunctionKind,
        modifiers: ZcVec::<'_, ZcModifierInvocation::<'_>>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        overrides: ZcOption::<'_, ZcOverrideSpecifier::<'_>>,
        parameters: ZcOption::<'_, ZcParameterList::<'_>>,
        return_parameters => ["returnParameters"]: ZcOption::<'_, ZcParameterList::<'_>>,
        scope: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        state_mutability => ["stateMutability"]: ZcStateMutability,
        _virtual => ["virtual"]: ZcOption::<'_, ZcBool::<'_>>,
        visibility: ZcVisibility,
    }

    pub struct ImportDirective {
        absolute_path => ["absolutePath"]: ZcStr::<'_>,
        file: ZcStr::<'_>,
        id: ZcIsize::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        scope: ZcIsize::<'_>,
        source_unit => ["sourceUnit"]: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        symbol_aliases => ["symbolAliases"]: ZcVec::<'_, ZcSymbolAliases::<'_>>,
        unit_alias => ["unitAlias"]: ZcStr::<'_>,
    }

    pub struct PragmaDirective {
        id: ZcIsize::<'_>,
        literals: ZcVec::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct StructDefinition {
        canonical_name => ["canonicalName"]: ZcStr::<'_>,
        id: ZcIsize::<'_>,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        members: ZcVec::<'_, ZcVariableDeclaration::<'_>>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        scope: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        visibility: ZcVisibility,
    }

    pub struct UserDefinedValueTypeDefinition {
        canonical_name => ["canonicalName"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        underlying_type => ["underlyingType"]: ZcTypeName::<'_>,
    }

    pub struct UsingForDirective {
        function_list => ["functionList"]: ZcOption::<'_, serde_json::Value>,
        global: ZcOption::<'_, ZcBool::<'_>>,
        id: ZcIsize::<'_>,
        library_name => ["libraryName"]: ZcOption::<'_, ZcLibraryName::<'_>>,
        src: ZcStr::<'_>,
        type_name => ["typeName"]: ZcOption::<'_, ZcTypeName::<'_>>,
    }

    pub struct VariableDeclaration {
        base_functions => ["baseFunctions"]: ZcOption::<'_, ZcVec::<'_, ZcIsize::<'_>>>,
        constant: ZcBool::<'_>,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        function_selector => ["functionSelector"]: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        indexed: ZcOption::<'_, ZcBool::<'_>>,
        mutability: ZcOption::<'_, ZcMutability>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        overrides: ZcOption::<'_, ZcOverrideSpecifier::<'_>>,
        scope: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        state_variable => ["stateVariable"]: ZcBool::<'_>,
        storage_location => ["storageLocation"]: ZcStorageLocation,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        type_name => ["typeName"]: ZcOption::<'_, ZcTypeName::<'_>>,
        value: ZcOption::<'_, ZcExpression::<'_>>,
        visibility: ZcVisibility,
    }

    pub struct StructuredDocumentation {
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        text: ZcStr::<'_>,
    }

    pub struct ParameterList {
        id: ZcIsize::<'_>,
        parameters: ZcVec::<'_, ZcVariableDeclaration::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct InheritanceSpecifier {
        arguments: ZcOption::<'_, ZcVec::<'_, ZcExpression::<'_>>>,
        base_name => ["baseName"]: ZcBaseName::<'_>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub enum ContractKind {
        simple:
        Contract,
        Interface,
        Library,
    }

    pub enum BaseNode {
        with_value:
        EnumDefinition,
        ErrorDefinition,
        FunctionDefinition,
        StructDefinition,
        UserDefinedValueTypeDefinition,
        UsingForDirective,
        VariableDeclaration,
        EventDefinition,
        ModifierDefinition,
    }

    pub struct ModifierDefinition {
        base_modifiers => ["baseModifiers"]: ZcOption::<'_, ZcVec::<'_, ZcIsize::<'_>>>,
        body: ZcBlock::<'_>,
        documentation: ZcOption::<'_, ZcStructuredDocumentation::<'_>>,
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        overrides: ZcOption::<'_, ZcOverrideSpecifier::<'_>>,
        parameters: ZcParameterList::<'_>,
        src: ZcStr::<'_>,
        _virtual => ["virtual"]: ZcOption::<'_, ZcBool::<'_>>,
        visibility: ZcVisibility,
    }

    pub struct Block {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        statements: ZcOption::<'_, ZcVec::<'_, ZcStatement::<'_>>>,
    }

    pub enum FunctionKind {
        simple:
        Function,
        Receive,
        Constructor,
        Fallback,
        FreeFunction,
    }



    pub struct ModifierInvocation {
        arguments: ZcOption::<'_, ZcVec::<'_, ZcExpression::<'_>>>,
        id: ZcIsize::<'_>,
        kind: ZcOption::<'_, ZcModifierKind>,
        modifier_name => ["modifierName"]: ZcModifierName::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct OverrideSpecifier {
        id: ZcIsize::<'_>,
        overrides: ZcVec::<'_, ZcOverrides::<'_>>,
        src: ZcStr::<'_>,
    }

    pub enum StateMutability {
        simple:
        Payable,
        Pure,
        Nonpayable,
        View,
    }

    pub enum Visibility {
        simple:
        External,
        Public,
        Internal,
        Private,
    }

    pub struct SymbolAliases {
        foreign: ZcIdentifier::<'_>,
        local: ZcOption::<'_, ZcStr::<'_>>,
        name_location => ["nameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
    }

    pub enum TypeName {
        with_value:
        ArrayTypeName,
        ElementaryTypeName,
        FunctionTypeName,
        Mapping,
        UserDefinedTypeName,
    }

    pub struct ArrayTypeName {
        base_type => ["baseType"]: ZcTypeName::<'_>,
        id: ZcIsize::<'_>,
        length: ZcOption::<'_, ZcExpression::<'_>>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct ElementaryTypeName {
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        src: ZcStr::<'_>,
        state_mutability => ["stateMutability"]: ZcOption::<'_, ZcStateMutability>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct FunctionTypeName {
        id: ZcIsize::<'_>,
        parameter_types => ["parameterTypes"]: ZcParameterList::<'_>,
        return_parameter_types => ["returnParameterTypes"]: ZcParameterList::<'_>,
        src: ZcStr::<'_>,
        state_mutability => ["stateMutability"]: ZcStateMutability,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        visibility: ZcVisibility,
    }

    pub struct Mapping {
        id: ZcIsize::<'_>,
        key_name => ["keyName"]: ZcOption::<'_, ZcStr::<'_>>,
        key_name_location => ["keyNameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        key_type => ["keyType"]: ZcTypeName::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        value_name => ["valueName"]: ZcOption::<'_, ZcStr::<'_>>,
        value_name_location => ["valueNameLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        value_type => ["valueType"]: ZcTypeName::<'_>,
    }

    pub struct UserDefinedTypeName {
        id: ZcIsize::<'_>,
        name: ZcOption::<'_, ZcStr::<'_>>,
        path_node => ["pathNode"]: ZcOption::<'_, ZcIdentifierPath::<'_>>,
        referenced_declaration => ["referencedDeclaration"]: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub enum LibraryName {
        with_value:
        UserDefinedTypeName,
        IdentifierPath,
    }

    pub struct IdentifierPath {
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        name_locations => ["nameLocations"]: ZcOption::<'_, ZcVec::<'_, ZcStr::<'_>>>,
        referenced_declaration => ["referencedDeclaration"]: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub enum Mutability {
        simple:
        Mutable,
        Immutable,
        Constant,
    }

    pub enum StorageLocation {
        simple:
        Calldata,
        Default,
        Memory,
        Storage,
    }

    pub struct TypeDescriptions {
        type_identifier => ["typeIdentifier"]: ZcOption::<'_, ZcStr::<'_>>,
        type_string => ["typeString"]: ZcOption::<'_, ZcStr::<'_>>,
    }

    pub enum BaseName {
        with_value:
        UserDefinedTypeName,
        IdentifierPath,
        Fallback,
    }

    pub struct Fallback {

    }

    pub enum ModifierKind {
        simple:
        ModifierInvocation,
        BaseConstructorSpecifier,
    }

    pub enum ModifierName {
        with_value:
        Identifier,
        IdentifierPath,
    }

    pub enum Overrides {
        with_value:
        UserDefinedTypeName,
        IdentifierPath,
    }

    pub enum Body {
        with_value:
        Block,
        Break,
        Continue,
        DoWhileStatement,
        EmitStatement,
        ExpressionStatement,
        ForStatement,
        IfStatement,
        InlineAssembly,
        PlaceholderStatement,
        Return,
        RevertStatement,
        TryStatement,
        UncheckedBlock,
        VariableDeclarationStatement,
        WhileStatement,
    }

    pub enum Statement {
        with_value:
        Block,
        Break,
        Continue,
        DoWhileStatement,
        EmitStatement,
        ExpressionStatement,
        ForStatement,
        IfStatement,
        InlineAssembly,
        PlaceholderStatement,
        Return,
        RevertStatement,
        TryStatement,
        UncheckedBlock,
        VariableDeclarationStatement,
        WhileStatement,
    }

    pub struct Break {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct Continue {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct DoWhileStatement {
        body: ZcBody::<'_>,
        condition: ZcOption::<'_, ZcExpression::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct EmitStatement {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        event_call => ["eventCall"]: ZcFunctionCall::<'_>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct ExpressionStatement {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        expression: ZcOption::<'_, ZcExpression::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct ForStatement {
        body: ZcBody::<'_>,
        condition: ZcOption::<'_, ZcExpression::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        initialization_expression => ["initializationExpression"]: ZcOption::<'_, ZcInitializationExpression::<'_>>,
        loop_expression => ["loopExpression"]: ZcOption::<'_, ZcExpressionStatement::<'_>>,
        src: ZcStr::<'_>,
    }

    pub enum InitializationExpression {
        with_value:
        ExpressionStatement,
        VariableDeclarationStatement,
    }

    pub struct IfStatement {
        condition: ZcOption::<'_, ZcExpression::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        false_body => ["falseBody"]: ZcOption::<'_, ZcFalseBody::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        true_body => ["trueBody"]: ZcFalseBody::<'_>, // TODO: Made it right
    }

    pub enum FalseBody {
        with_value:
        Block,
        Break,
        Continue,
        DoWhileStatement,
        EmitStatement,
        ExpressionStatement,
        ForStatement,
        IfStatement,
        PlaceholderStatement,
        Return,
        RevertStatement,
        TryStatement,
        UncheckedBlock,
        VariableDeclarationStatement,
        WhileStatement,
    }

    pub struct InlineAssembly {
        ast => ["AST"]: ZcOption::<'_, ZcYulBlock::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        evm_version => ["evmVersion"]: ZcOption::<'_, ZcEvmVersion>,
        external_references => ["externalReferences"]: ZcVec::<'_, ZcExternalReferenceCompatible::<'_>>,
        flags: ZcOption::<'_, ZcVec::<'_,ZcStr::<'_>>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub enum EvmVersion {
        simple:
        Homestead,
        TangerineWhistle,
        SpuriousDragon,
        Byzantium,
        Constantinople,
        Petersburg,
        Istanbul,
        Berlin,
        London,
        Paris,
        Shanghai,
        Cancun,
    }

    pub enum ExternalReferenceCompatible {
        with_value:
        ExternalReference,
        ExternalReferenceOld,
    }

    pub struct ExternalReference {
        declaration: ZcIsize::<'_>,
        is_offset => ["isOffset"]: ZcBool::<'_>,
        is_slot => ["isSlot"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        suffix: ZcOption::<'_, ZcSuffix>,
        value_size => ["valueSize"]: ZcI32::<'_>,
    }

    pub enum Suffix {
        simple:
        Offset,
        Slot,
        Length,
    }

    pub struct ExternalReferenceOld {
        inner: ZcHashMap::<'_, ZcExternalReference::<'_>>,
    }

    pub struct PlaceholderStatement {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct Return {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        expression: ZcOption::<'_, ZcExpression::<'_>>,
        function_return_parameters => ["functionReturnParameters"]: ZcOption::<'_, ZcIsize::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct RevertStatement {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        error_call => ["errorCall"]: ZcFunctionCall::<'_>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct TryStatement {
        clauses: ZcVec::<'_, ZcTryCatchClause::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        external_call => ["externalCall"]: ZcFunctionCall::<'_>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct TryCatchClause {
        block: ZcBlock::<'_>,
        error_name => ["errorName"]: ZcStr::<'_>,
        id: ZcIsize::<'_>,
        parameters: ZcOption::<'_, ZcParameterList::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct UncheckedBlock {
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
        statements: ZcVec::<'_, ZcStatement::<'_>>,
    }

    pub struct VariableDeclarationStatement {
        assignments: ZcVec::<'_,ZcOption::<'_, ZcIsize::<'_>>>,
        declarations: ZcVec::<'_,ZcOption::<'_, ZcVariableDeclaration::<'_>>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        initial_value => ["initialValue"]: ZcOption::<'_, ZcExpression::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct WhileStatement {
        body: ZcBody::<'_>,
        condition: ZcOption::<'_, ZcExpression::<'_>>,
        documentation: ZcOption::<'_, ZcStr::<'_>>,
        id: ZcIsize::<'_>,
        src: ZcStr::<'_>,
    }

    pub enum Expression {
        with_value:
        Assignment,
        BinaryOperation,
        Conditional,
        ElementaryTypeNameExpression,
        FunctionCall,
        FunctionCallOptions,
        Identifier,
        IndexAccess,
        IndexRangeAccess,
        Literal,
        MemberAccess,
        NewExpression,
        TupleExpression,
        UnaryOperation,
    }

    pub struct Assignment {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        left_hand_side => ["leftHandSide"]: ZcExpression::<'_>,
        operator: ZcStr::<'_>,
        right_hand_side => ["rightHandSide"]: ZcExpression::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct BinaryOperation {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        common_type => ["commonType"]: ZcTypeDescriptions::<'_>,
        function: ZcOption::<'_, ZcIsize::<'_>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        left_expression => ["leftExpression"]: ZcExpression::<'_>,
        operator: ZcStr::<'_>,
        right_expression => ["rightExpression"]: ZcExpression::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct Conditional {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        condition: ZcExpression::<'_>,
        false_expression => ["falseExpression"]: ZcExpression::<'_>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        true_expression => ["trueExpression"]: ZcExpression::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct ElementaryTypeNameExpression {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        type_name => ["typeName"]: ZcCompatabilityTypeName::<'_>,
    }

    pub enum CompatabilityTypeName {
        with_value:
        ElementaryTypeName,
        Name,
    }

    pub struct Name {
        inner: ZcStr::<'_>,
    }

    pub struct FunctionCall {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        arguments: ZcVec::<'_, ZcExpression::<'_>>,
        expression: ZcExpression::<'_>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        kind: ZcFunctionCallKind,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        name_locations => ["nameLocations"]: ZcOption::<'_, ZcVec::<'_, ZcStr::<'_>>>,
        names: ZcVec::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        try_call => ["tryCall"]: ZcOption::<'_, ZcBool::<'_>>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub enum FunctionCallKind {
        simple:
        FunctionCall,
        TypeConversion,
        StructConstructorCall,
    }

    pub struct FunctionCallOptions {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        expression: ZcExpression::<'_>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcOption::<'_, ZcBool::<'_>>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        names: ZcVec::<'_, ZcStr::<'_>>,
        options: ZcVec::<'_, ZcExpression::<'_>>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct Identifier {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        id: ZcIsize::<'_>,
        name: ZcStr::<'_>,
        overloaded_declarations => ["overloadedDeclarations"]: ZcVec::<'_, ZcIsize::<'_>>,
        referenced_declaration => ["referencedDeclaration"]: ZcOption::<'_, ZcIsize::<'_>>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct IndexAccess {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        base_expression => ["baseExpression"]: ZcExpression::<'_>,
        id: ZcIsize::<'_>,
        index_expression => ["indexExpression"]: ZcOption::<'_, ZcExpression::<'_>>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct IndexRangeAccess {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        base_expression => ["baseExpression"]: ZcExpression::<'_>,
        end_expression => ["endExpression"]: ZcOption::<'_, ZcExpression::<'_>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        start_expression => ["startExpression"]: ZcOption::<'_, ZcExpression::<'_>>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct Literal {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        hex_value => ["hexValue"]: ZcStr::<'_>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        kind: ZcLiteralKind,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        subdenomination: ZcOption::<'_, ZcSubdenomination>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        value: ZcOption::<'_, ZcStr::<'_>>,
    }

    pub enum LiteralKind {
        simple:
        Bool,
        Number,
        String,
        HexString,
        UnicodeString,
    }

    pub enum Subdenomination {
        simple:
        Weeks,
        Days,
        Hours,
        Minutes,
        Seconds,
        Wei,
        Gwei,
        Ether,
        Finney,
        Szabo,
    }

    pub struct MemberAccess {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        expression: ZcExpression::<'_>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        member_location => ["memberLocation"]: ZcOption::<'_, ZcStr::<'_>>,
        member_name => ["memberName"]: ZcStr::<'_>,
        referenced_declaration => ["referencedDeclaration"]: ZcOption::<'_, ZcIsize::<'_>>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct NewExpression {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcOption::<'_, ZcBool::<'_>>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
        type_name => ["typeName"]: ZcTypeName::<'_>,
    }

    pub struct TupleExpression {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        components: ZcVec::<'_, ZcOption::<'_, ZcExpression::<'_>>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_inline_array => ["isInlineArray"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        src: ZcStr::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub struct UnaryOperation {
        argument_types => ["argumentTypes"]: ZcOption::<'_, ZcVec::<'_, ZcTypeDescriptions::<'_>>>,
        function: ZcOption::<'_, ZcIsize::<'_>>,
        id: ZcIsize::<'_>,
        is_constant => ["isConstant"]: ZcBool::<'_>,
        is_lvalue => ["isLValue"]: ZcBool::<'_>,
        is_pure => ["isPure"]: ZcBool::<'_>,
        l_value_requested => ["lValueRequested"]: ZcBool::<'_>,
        operator: ZcStr::<'_>,
        prefix: ZcBool::<'_>,
        src: ZcStr::<'_>,
        sub_expression => ["subExpression"]: ZcExpression::<'_>,
        type_descriptions => ["typeDescriptions"]: ZcTypeDescriptions::<'_>,
    }

    pub enum YulStatement {
        with_value:
        YulAssignment,
        YulBlock,
        YulBreak,
        YulContinue,
        YulExpressionStatement,
        YulLeave,
        YulForLoop,
        YulFunctionDefinition,
        YulIf,
        YulSwitch,
        YulVariableDeclaration,
    }

    pub struct YulAssignment {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        value: ZcYulExpression::<'_>,
        variable_names => ["variableNames"]: ZcVec::<'_, ZcYulIdentifier::<'_>>,
    }

    pub struct YulBlock {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        statements: ZcVec::<'_, ZcYulStatement::<'_>>,
    }

    pub struct YulBreak {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulContinue {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulExpressionStatement {
        expression: ZcYulExpression::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulLeave {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulForLoop {
        body: ZcYulBlock::<'_>,
        condition: ZcYulExpression::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        post: ZcYulBlock::<'_>,
        pre: ZcYulBlock::<'_>,
        src: ZcStr::<'_>,
    }

    pub struct YulFunctionDefinition {
        body: ZcYulBlock::<'_>,
        name: ZcStr::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        parameters: ZcOption::<'_, ZcVec::<'_, ZcYulTypedName::<'_>>>,
        return_variables: ZcOption::<'_, ZcVec::<'_, ZcYulTypedName::<'_>>>,
        src: ZcStr::<'_>,
    }

    pub struct YulIf {
        body: ZcYulBlock::<'_>,
        condition: ZcYulExpression::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulSwitch {
        cases: ZcVec::<'_, ZcYulCase::<'_>>,
        expression: ZcYulExpression::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulCase {
        body: ZcYulBlock::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        value: ZcCaseValue::<'_>,
    }

    pub enum CaseValue {
        with_value:
        YulDefault,
        YulLiteral,
    }

    pub struct YulDefault {
        inner: ZcStr::<'_>,
    }

    pub struct YulVariableDeclaration {
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        value => ["value"]: ZcOption::<'_, ZcYulExpression::<'_>>, // Handling `null` as `ZcOption::<'_, YulExpression>`
        variables: ZcVec::<'_, ZcYulTypedName::<'_>>,
    }

    pub struct YulTypedName {
        name: ZcStr::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        r#type: ZcStr::<'_>, // Use `r#type` to avoid conflict with Rust's `type` keyword
    }

    pub enum YulExpression {
        with_value:
        YulFunctionCall,
        YulIdentifier,
        YulLiteral,
    }

    pub struct YulFunctionCall {
        arguments: ZcVec::<'_, ZcYulExpression::<'_>>,
        function_name => ["functionName"]: ZcYulIdentifier::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub struct YulIdentifier {
        name: ZcStr::<'_>,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
    }

    pub enum YulLiteral {
        with_value:
        YulLiteralValue,
        YulLiteralHexValue,
    }

    pub struct YulLiteralValue {
        kind: ZcYulLiteralKind,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        r#type: ZcStr::<'_>, // Use `r#type` to avoid conflict with Rust's `type` keyword
        value: ZcStr::<'_>,
    }

    pub enum YulLiteralKind {
        simple:
        String,
        Number,
        Bool,
    }

    pub struct YulLiteralHexValue {
        hex_value => ["hexValue"]: ZcStr::<'_>,
        kind: ZcYulLiteralKind,
        native_src => ["nativeSrc"]: ZcOption::<'_, ZcStr::<'_>>,
        src: ZcStr::<'_>,
        r#type: ZcStr::<'_>, // Use `r#type` to avoid conflict with Rust's `type` keyword
        value => ["value"]: ZcOption::<'_, ZcStr::<'_>>, // Optional field
    }
}
