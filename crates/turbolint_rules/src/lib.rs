#[cfg(test)]
mod test_helpers;

mod accessor_pairs;
mod array_bracket_newline;
mod array_bracket_spacing;
mod array_callback_return;
mod array_element_newline;
mod arrow_body_style;
mod arrow_parens;
mod arrow_spacing;
mod block_scoped_var;
mod block_spacing;
mod brace_style;
mod callback_return;
mod camelcase;
mod capitalized_comments;
mod class_methods_use_this;
mod comma_dangle;
mod comma_spacing;
mod comma_style;
mod complexity;
mod computed_property_spacing;
mod consistent_return;
mod consistent_this;
mod constructor_super;
mod curly;
mod default_case;
mod default_case_last;
mod default_param_last;
mod dot_location;
mod dot_notation;
mod eol_last;
mod eqeqeq;
mod for_direction;
mod func_call_spacing;
mod func_name_matching;
mod func_names;
mod func_style;
mod function_call_argument_newline;
mod function_paren_newline;
mod generator_star_spacing;
mod getter_return;
mod global_require;
mod grouped_accessor_pairs;
mod guard_for_in;
mod handle_callback_err;
mod id_blacklist;
mod id_denylist;
mod id_length;
mod id_match;
mod implicit_arrow_linebreak;
mod indent;
mod indent_legacy;
mod init_declarations;
mod jsx_quotes;
mod key_spacing;
mod keyword_spacing;
mod line_comment_position;
mod linebreak_style;
mod lines_around_comment;
mod lines_around_directive;
mod lines_between_class_members;
mod logical_assignment_operators;
mod max_classes_per_file;
mod max_depth;
mod max_len;
mod max_lines;
mod max_lines_per_function;
mod max_nested_callbacks;
mod max_params;
mod max_statements;
mod max_statements_per_line;
mod multiline_comment_style;
mod multiline_ternary;
mod new_cap;
mod newline_after_var;
mod newline_before_return;
mod newline_per_chained_call;
mod no_alert;
mod no_array_constructor;
mod no_async_promise_executor;
mod no_await_in_loop;
mod no_bitwise;
mod no_buffer_constructor;
mod no_caller;
mod no_case_declarations;
mod no_catch_shadow;
mod no_class_assign;
mod no_compare_neg_zero;
mod no_cond_assign;
mod no_confusing_arrow;
mod no_console;
mod no_const_assign;
mod no_constant_binary_expression;
mod no_constant_condition;
mod no_constructor_return;
mod no_continue;
mod no_control_regex;
mod no_debugger;
mod no_delete_var;
mod no_div_regex;
mod no_dupe_args;
mod no_dupe_class_members;
mod no_dupe_else_if;
mod no_dupe_keys;
mod no_duplicate_case;
mod no_duplicate_imports;
mod no_else_return;
mod no_empty;
mod no_empty_character_class;
mod no_empty_function;
mod no_empty_pattern;
mod no_empty_static_block;
mod no_eq_null;
mod no_eval;
mod no_ex_assign;
mod no_extend_native;
mod no_extra_bind;
mod no_extra_boolean_cast;
mod no_extra_label;
mod no_extra_parens;
mod no_extra_semi;
mod no_fallthrough;
mod no_floating_decimal;
mod no_func_assign;
mod no_global_assign;
mod no_implicit_coercion;
mod no_implicit_globals;
mod no_implied_eval;
mod no_import_assign;
mod no_inline_comments;
mod no_inner_declarations;
mod no_invalid_regexp;
mod no_invalid_this;
mod no_irregular_whitespace;
mod no_iterator;
mod no_label_var;
mod no_labels;
mod no_lone_blocks;
mod no_lonely_if;
mod no_loop_func;
mod no_loss_of_precision;
mod no_magic_numbers;
mod no_misleading_character_class;
mod no_mixed_operators;
mod no_mixed_requires;
mod no_mixed_spaces_and_tabs;
mod no_multi_assign;
mod no_multi_spaces;
mod no_multi_str;
mod no_multiple_empty_lines;
mod no_native_reassign;
mod no_negated_condition;
mod no_negated_in_lhs;
mod no_nested_ternary;
mod no_new;
mod no_new_func;
mod no_new_native_nonconstructor;
mod no_new_object;
mod no_new_require;
mod no_new_symbol;
mod no_new_wrappers;
mod no_nonoctal_decimal_escape;
mod no_obj_calls;
mod no_object_constructor;
mod no_octal;
mod no_octal_escape;
mod no_param_reassign;
mod no_path_concat;
mod no_plusplus;
mod no_process_env;
mod no_process_exit;
mod no_promise_executor_return;
mod no_proto;
mod no_prototype_builtins;
mod no_redeclare;
mod no_regex_spaces;
mod no_restricted_exports;
mod no_restricted_globals;
mod no_restricted_imports;
mod no_restricted_modules;
mod no_restricted_properties;
mod no_restricted_syntax;
mod no_return_assign;
mod no_return_await;
mod no_script_url;
mod no_self_assign;
mod no_self_compare;
mod no_sequences;
mod no_setter_return;
mod no_shadow;
mod no_shadow_restricted_names;
mod no_spaced_func;
mod no_sparse_arrays;
mod no_sync;
mod no_tabs;
mod no_template_curly_in_string;
mod no_ternary;
mod no_this_before_super;
mod no_throw_literal;
mod no_trailing_spaces;
mod no_undef;
mod no_undef_init;
mod no_undefined;
mod no_underscore_dangle;
mod no_unexpected_multiline;
mod no_unmodified_loop_condition;
mod no_unneeded_ternary;
mod no_unreachable;
mod no_unreachable_loop;
mod no_unsafe_finally;
mod no_unsafe_negation;
mod no_unsafe_optional_chaining;
mod no_unused_expressions;
mod no_unused_labels;
mod no_unused_private_class_members;
mod no_unused_vars;
mod no_use_before_define;
mod no_useless_assignment;
mod no_useless_backreference;
mod no_useless_call;
mod no_useless_catch;
mod no_useless_computed_key;
mod no_useless_concat;
mod no_useless_constructor;
mod no_useless_escape;
mod no_useless_rename;
mod no_useless_return;
mod no_var;
mod no_void;
mod no_warning_comments;
mod no_whitespace_before_property;
mod no_with;
mod nonblock_statement_body_position;
mod object_curly_newline;
mod object_curly_spacing;
mod object_property_newline;
mod object_shorthand;
mod one_var;
mod one_var_declaration_per_line;
mod operator_assignment;
mod operator_linebreak;
mod padded_blocks;
mod padding_line_between_statements;
mod prefer_arrow_callback;
mod prefer_const;
mod prefer_destructuring;
mod prefer_exponentiation_operator;
mod prefer_named_capture_group;
mod prefer_numeric_literals;
mod prefer_object_has_own;
mod prefer_object_spread;
mod prefer_promise_reject_errors;
mod prefer_regex_literals;
mod prefer_rest_params;
mod prefer_spread;
mod prefer_template;
mod preserve_caught_error;
mod quote_props;
mod quotes;
mod radix;
mod require_atomic_updates;
mod require_await;
mod require_unicode_regexp;
mod require_yield;
mod rest_spread_spacing;
mod semi;
mod semi_spacing;
mod semi_style;
mod sort_imports;
mod sort_keys;
mod sort_vars;
mod space_before_blocks;
mod space_before_function_paren;
mod space_in_parens;
mod space_infix_ops;
mod space_unary_ops;
mod spaced_comment;
mod strict;
mod switch_colon_spacing;
mod symbol_description;
mod template_curly_spacing;
mod template_tag_spacing;
mod unicode_bom;
mod use_isnan;
mod valid_typeof;
mod vars_on_top;
mod wrap_iife;
mod wrap_regex;
mod yield_star_spacing;
mod yoda;

use turbolint_core::Rule;

pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(accessor_pairs::AccessorPairs),
        Box::new(array_bracket_newline::ArrayBracketNewline),
        Box::new(array_bracket_spacing::ArrayBracketSpacing),
        Box::new(array_callback_return::ArrayCallbackReturn),
        Box::new(array_element_newline::ArrayElementNewline),
        Box::new(arrow_body_style::ArrowBodyStyle),
        Box::new(arrow_parens::ArrowParens),
        Box::new(arrow_spacing::ArrowSpacing),
        Box::new(block_scoped_var::BlockScopedVar),
        Box::new(block_spacing::BlockSpacing),
        Box::new(brace_style::BraceStyle),
        Box::new(callback_return::CallbackReturn),
        Box::new(camelcase::Camelcase),
        Box::new(capitalized_comments::CapitalizedComments),
        Box::new(class_methods_use_this::ClassMethodsUseThis),
        Box::new(comma_dangle::CommaDangle),
        Box::new(comma_spacing::CommaSpacing),
        Box::new(comma_style::CommaStyle),
        Box::new(complexity::Complexity),
        Box::new(computed_property_spacing::ComputedPropertySpacing),
        Box::new(consistent_return::ConsistentReturn),
        Box::new(consistent_this::ConsistentThis),
        Box::new(constructor_super::ConstructorSuper),
        Box::new(curly::Curly),
        Box::new(default_case::DefaultCase),
        Box::new(default_case_last::DefaultCaseLast),
        Box::new(default_param_last::DefaultParamLast),
        Box::new(dot_location::DotLocation),
        Box::new(dot_notation::DotNotation),
        Box::new(eol_last::EolLast),
        Box::new(eqeqeq::Eqeqeq),
        Box::new(for_direction::ForDirection),
        Box::new(func_call_spacing::FuncCallSpacing),
        Box::new(func_name_matching::FuncNameMatching),
        Box::new(func_names::FuncNames),
        Box::new(func_style::FuncStyle),
        Box::new(function_call_argument_newline::FunctionCallArgumentNewline),
        Box::new(function_paren_newline::FunctionParenNewline),
        Box::new(generator_star_spacing::GeneratorStarSpacing),
        Box::new(getter_return::GetterReturn),
        Box::new(global_require::GlobalRequire),
        Box::new(grouped_accessor_pairs::GroupedAccessorPairs),
        Box::new(guard_for_in::GuardForIn),
        Box::new(handle_callback_err::HandleCallbackErr),
        Box::new(id_blacklist::IdBlacklist),
        Box::new(id_denylist::IdDenylist),
        Box::new(id_length::IdLength),
        Box::new(id_match::IdMatch),
        Box::new(implicit_arrow_linebreak::ImplicitArrowLinebreak),
        Box::new(indent::Indent),
        Box::new(indent_legacy::IndentLegacy),
        Box::new(init_declarations::InitDeclarations),
        Box::new(jsx_quotes::JsxQuotes),
        Box::new(key_spacing::KeySpacing),
        Box::new(keyword_spacing::KeywordSpacing),
        Box::new(line_comment_position::LineCommentPosition),
        Box::new(linebreak_style::LinebreakStyle),
        Box::new(lines_around_comment::LinesAroundComment),
        Box::new(lines_around_directive::LinesAroundDirective),
        Box::new(lines_between_class_members::LinesBetweenClassMembers),
        Box::new(logical_assignment_operators::LogicalAssignmentOperators),
        Box::new(max_classes_per_file::MaxClassesPerFile),
        Box::new(max_depth::MaxDepth),
        Box::new(max_len::MaxLen),
        Box::new(max_lines::MaxLines),
        Box::new(max_lines_per_function::MaxLinesPerFunction),
        Box::new(max_nested_callbacks::MaxNestedCallbacks),
        Box::new(max_params::MaxParams),
        Box::new(max_statements::MaxStatements),
        Box::new(max_statements_per_line::MaxStatementsPerLine),
        Box::new(multiline_comment_style::MultilineCommentStyle),
        Box::new(multiline_ternary::MultilineTernary),
        Box::new(new_cap::NewCap),
        Box::new(newline_after_var::NewlineAfterVar),
        Box::new(newline_before_return::NewlineBeforeReturn),
        Box::new(newline_per_chained_call::NewlinePerChainedCall),
        Box::new(no_alert::NoAlert),
        Box::new(no_array_constructor::NoArrayConstructor),
        Box::new(no_async_promise_executor::NoAsyncPromiseExecutor),
        Box::new(no_await_in_loop::NoAwaitInLoop),
        Box::new(no_bitwise::NoBitwise),
        Box::new(no_buffer_constructor::NoBufferConstructor),
        Box::new(no_caller::NoCaller),
        Box::new(no_case_declarations::NoCaseDeclarations),
        Box::new(no_catch_shadow::NoCatchShadow),
        Box::new(no_class_assign::NoClassAssign),
        Box::new(no_compare_neg_zero::NoCompareNegZero),
        Box::new(no_cond_assign::NoCondAssign),
        Box::new(no_confusing_arrow::NoConfusingArrow),
        Box::new(no_console::NoConsole),
        Box::new(no_const_assign::NoConstAssign),
        Box::new(no_constant_binary_expression::NoConstantBinaryExpression),
        Box::new(no_constant_condition::NoConstantCondition),
        Box::new(no_constructor_return::NoConstructorReturn),
        Box::new(no_continue::NoContinue),
        Box::new(no_control_regex::NoControlRegex),
        Box::new(no_debugger::NoDebugger),
        Box::new(no_delete_var::NoDeleteVar),
        Box::new(no_div_regex::NoDivRegex),
        Box::new(no_dupe_args::NoDupeArgs),
        Box::new(no_dupe_class_members::NoDupeClassMembers),
        Box::new(no_dupe_else_if::NoDupeElseIf),
        Box::new(no_dupe_keys::NoDupeKeys),
        Box::new(no_duplicate_case::NoDuplicateCase),
        Box::new(no_duplicate_imports::NoDuplicateImports),
        Box::new(no_else_return::NoElseReturn),
        Box::new(no_empty::NoEmpty),
        Box::new(no_empty_character_class::NoEmptyCharacterClass),
        Box::new(no_empty_function::NoEmptyFunction),
        Box::new(no_empty_pattern::NoEmptyPattern),
        Box::new(no_empty_static_block::NoEmptyStaticBlock),
        Box::new(no_eq_null::NoEqNull),
        Box::new(no_eval::NoEval),
        Box::new(no_ex_assign::NoExAssign),
        Box::new(no_extend_native::NoExtendNative),
        Box::new(no_extra_bind::NoExtraBind),
        Box::new(no_extra_boolean_cast::NoExtraBooleanCast),
        Box::new(no_extra_label::NoExtraLabel),
        Box::new(no_extra_parens::NoExtraParens),
        Box::new(no_extra_semi::NoExtraSemi),
        Box::new(no_fallthrough::NoFallthrough),
        Box::new(no_floating_decimal::NoFloatingDecimal),
        Box::new(no_func_assign::NoFuncAssign),
        Box::new(no_global_assign::NoGlobalAssign),
        Box::new(no_implicit_coercion::NoImplicitCoercion),
        Box::new(no_implicit_globals::NoImplicitGlobals),
        Box::new(no_implied_eval::NoImpliedEval),
        Box::new(no_import_assign::NoImportAssign),
        Box::new(no_inline_comments::NoInlineComments),
        Box::new(no_inner_declarations::NoInnerDeclarations),
        Box::new(no_invalid_regexp::NoInvalidRegexp),
        Box::new(no_invalid_this::NoInvalidThis),
        Box::new(no_irregular_whitespace::NoIrregularWhitespace),
        Box::new(no_iterator::NoIterator),
        Box::new(no_label_var::NoLabelVar),
        Box::new(no_labels::NoLabels),
        Box::new(no_lone_blocks::NoLoneBlocks),
        Box::new(no_lonely_if::NoLonelyIf),
        Box::new(no_loop_func::NoLoopFunc),
        Box::new(no_loss_of_precision::NoLossOfPrecision),
        Box::new(no_magic_numbers::NoMagicNumbers),
        Box::new(no_misleading_character_class::NoMisleadingCharacterClass),
        Box::new(no_mixed_operators::NoMixedOperators),
        Box::new(no_mixed_requires::NoMixedRequires),
        Box::new(no_mixed_spaces_and_tabs::NoMixedSpacesAndTabs),
        Box::new(no_multi_assign::NoMultiAssign),
        Box::new(no_multi_spaces::NoMultiSpaces),
        Box::new(no_multi_str::NoMultiStr),
        Box::new(no_multiple_empty_lines::NoMultipleEmptyLines),
        Box::new(no_native_reassign::NoNativeReassign),
        Box::new(no_negated_condition::NoNegatedCondition),
        Box::new(no_negated_in_lhs::NoNegatedInLhs),
        Box::new(no_nested_ternary::NoNestedTernary),
        Box::new(no_new::NoNew),
        Box::new(no_new_func::NoNewFunc),
        Box::new(no_new_native_nonconstructor::NoNewNativeNonconstructor),
        Box::new(no_new_object::NoNewObject),
        Box::new(no_new_require::NoNewRequire),
        Box::new(no_new_symbol::NoNewSymbol),
        Box::new(no_new_wrappers::NoNewWrappers),
        Box::new(no_nonoctal_decimal_escape::NoNonOctalDecimalEscape),
        Box::new(no_obj_calls::NoObjCalls),
        Box::new(no_object_constructor::NoObjectConstructor),
        Box::new(no_octal::NoOctal),
        Box::new(no_octal_escape::NoOctalEscape),
        Box::new(no_param_reassign::NoParamReassign),
        Box::new(no_path_concat::NoPathConcat),
        Box::new(no_plusplus::NoPlusplus),
        Box::new(no_process_env::NoProcessEnv),
        Box::new(no_process_exit::NoProcessExit),
        Box::new(no_promise_executor_return::NoPromiseExecutorReturn),
        Box::new(no_proto::NoProto),
        Box::new(no_prototype_builtins::NoPrototypeBuiltins),
        Box::new(no_redeclare::NoRedeclare),
        Box::new(no_regex_spaces::NoRegexSpaces),
        Box::new(no_restricted_exports::NoRestrictedExports),
        Box::new(no_restricted_globals::NoRestrictedGlobals),
        Box::new(no_restricted_imports::NoRestrictedImports),
        Box::new(no_restricted_modules::NoRestrictedModules),
        Box::new(no_restricted_properties::NoRestrictedProperties),
        Box::new(no_restricted_syntax::NoRestrictedSyntax),
        Box::new(no_return_assign::NoReturnAssign),
        Box::new(no_return_await::NoReturnAwait),
        Box::new(no_script_url::NoScriptUrl),
        Box::new(no_self_assign::NoSelfAssign),
        Box::new(no_self_compare::NoSelfCompare),
        Box::new(no_sequences::NoSequences),
        Box::new(no_setter_return::NoSetterReturn),
        Box::new(no_shadow::NoShadow),
        Box::new(no_shadow_restricted_names::NoShadowRestrictedNames),
        Box::new(no_spaced_func::NoSpacedFunc),
        Box::new(no_sparse_arrays::NoSparseArrays),
        Box::new(no_sync::NoSync),
        Box::new(no_tabs::NoTabs),
        Box::new(no_template_curly_in_string::NoTemplateCurlyInString),
        Box::new(no_ternary::NoTernary),
        Box::new(no_this_before_super::NoThisBeforeSuper),
        Box::new(no_throw_literal::NoThrowLiteral),
        Box::new(no_trailing_spaces::NoTrailingSpaces),
        Box::new(no_undef::NoUndef),
        Box::new(no_undef_init::NoUndefInit),
        Box::new(no_undefined::NoUndefined),
        Box::new(no_underscore_dangle::NoUnderscoreDangle),
        Box::new(no_unexpected_multiline::NoUnexpectedMultiline),
        Box::new(no_unmodified_loop_condition::NoUnmodifiedLoopCondition),
        Box::new(no_unneeded_ternary::NoUnneededTernary),
        Box::new(no_unreachable::NoUnreachable),
        Box::new(no_unreachable_loop::NoUnreachableLoop),
        Box::new(no_unsafe_finally::NoUnsafeFinally),
        Box::new(no_unsafe_negation::NoUnsafeNegation),
        Box::new(no_unsafe_optional_chaining::NoUnsafeOptionalChaining),
        Box::new(no_unused_expressions::NoUnusedExpressions),
        Box::new(no_unused_labels::NoUnusedLabels),
        Box::new(no_unused_private_class_members::NoUnusedPrivateClassMembers),
        Box::new(no_unused_vars::NoUnusedVars),
        Box::new(no_use_before_define::NoUseBeforeDefine),
        Box::new(no_useless_assignment::NoUselessAssignment),
        Box::new(no_useless_backreference::NoUselessBackreference),
        Box::new(no_useless_call::NoUselessCall),
        Box::new(no_useless_catch::NoUselessCatch),
        Box::new(no_useless_computed_key::NoUselessComputedKey),
        Box::new(no_useless_concat::NoUselessConcat),
        Box::new(no_useless_constructor::NoUselessConstructor),
        Box::new(no_useless_escape::NoUselessEscape),
        Box::new(no_useless_rename::NoUselessRename),
        Box::new(no_useless_return::NoUselessReturn),
        Box::new(no_var::NoVar),
        Box::new(no_void::NoVoid),
        Box::new(no_warning_comments::NoWarningComments),
        Box::new(no_whitespace_before_property::NoWhitespaceBeforeProperty),
        Box::new(no_with::NoWith),
        Box::new(nonblock_statement_body_position::NonblockStatementBodyPosition),
        Box::new(object_curly_newline::ObjectCurlyNewline),
        Box::new(object_curly_spacing::ObjectCurlySpacing),
        Box::new(object_property_newline::ObjectPropertyNewline),
        Box::new(object_shorthand::ObjectShorthand),
        Box::new(one_var::OneVar),
        Box::new(one_var_declaration_per_line::OneVarDeclarationPerLine),
        Box::new(operator_assignment::OperatorAssignment),
        Box::new(operator_linebreak::OperatorLinebreak),
        Box::new(padded_blocks::PaddedBlocks),
        Box::new(padding_line_between_statements::PaddingLineBetweenStatements),
        Box::new(prefer_arrow_callback::PreferArrowCallback),
        Box::new(prefer_const::PreferConst),
        Box::new(prefer_destructuring::PreferDestructuring),
        Box::new(prefer_exponentiation_operator::PreferExponentiationOperator),
        Box::new(prefer_named_capture_group::PreferNamedCaptureGroup),
        Box::new(prefer_numeric_literals::PreferNumericLiterals),
        Box::new(prefer_object_has_own::PreferObjectHasOwn),
        Box::new(prefer_object_spread::PreferObjectSpread),
        Box::new(prefer_promise_reject_errors::PreferPromiseRejectErrors),
        Box::new(prefer_regex_literals::PreferRegexLiterals),
        Box::new(prefer_rest_params::PreferRestParams),
        Box::new(prefer_spread::PreferSpread),
        Box::new(prefer_template::PreferTemplate),
        Box::new(preserve_caught_error::PreserveCaughtError),
        Box::new(quote_props::QuoteProps),
        Box::new(quotes::Quotes),
        Box::new(radix::Radix),
        Box::new(require_atomic_updates::RequireAtomicUpdates),
        Box::new(require_await::RequireAwait),
        Box::new(require_unicode_regexp::RequireUnicodeRegexp),
        Box::new(require_yield::RequireYield),
        Box::new(rest_spread_spacing::RestSpreadSpacing),
        Box::new(semi::Semi),
        Box::new(semi_spacing::SemiSpacing),
        Box::new(semi_style::SemiStyle),
        Box::new(sort_imports::SortImports),
        Box::new(sort_keys::SortKeys),
        Box::new(sort_vars::SortVars),
        Box::new(space_before_blocks::SpaceBeforeBlocks),
        Box::new(space_before_function_paren::SpaceBeforeFunctionParen),
        Box::new(space_in_parens::SpaceInParens),
        Box::new(space_infix_ops::SpaceInfixOps),
        Box::new(space_unary_ops::SpaceUnaryOps),
        Box::new(spaced_comment::SpacedComment),
        Box::new(strict::Strict),
        Box::new(switch_colon_spacing::SwitchColonSpacing),
        Box::new(symbol_description::SymbolDescription),
        Box::new(template_curly_spacing::TemplateCurlySpacing),
        Box::new(template_tag_spacing::TemplateTagSpacing),
        Box::new(unicode_bom::UnicodeBom),
        Box::new(use_isnan::UseIsnan),
        Box::new(valid_typeof::ValidTypeof),
        Box::new(vars_on_top::VarsOnTop),
        Box::new(wrap_iife::WrapIife),
        Box::new(wrap_regex::WrapRegex),
        Box::new(yield_star_spacing::YieldStarSpacing),
        Box::new(yoda::Yoda),
    ]
}
