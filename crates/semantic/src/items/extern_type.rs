use defs::ids::{ExternTypeId, GenericParamId, LanguageElementId};
use diagnostics::Diagnostics;
use diagnostics_proc_macros::DebugWithDb;

use super::generics::semantic_generic_params;
use crate::db::SemanticGroup;
use crate::diagnostic::SemanticDiagnostics;
use crate::SemanticDiagnostic;

#[cfg(test)]
#[path = "extern_type_test.rs"]
mod test;

// Declaration.
#[derive(Clone, Debug, PartialEq, Eq, DebugWithDb)]
#[debug_db(dyn SemanticGroup + 'static)]
pub struct ExternTypeDeclarationData {
    diagnostics: Diagnostics<SemanticDiagnostic>,
    generic_params: Vec<GenericParamId>,
}

// Selectors.
/// Query implementation of [crate::db::SemanticGroup::extern_type_declaration_diagnostics].
pub fn extern_type_declaration_diagnostics(
    db: &dyn SemanticGroup,
    extern_type_id: ExternTypeId,
) -> Diagnostics<SemanticDiagnostic> {
    db.priv_extern_type_declaration_data(extern_type_id)
        .map(|data| data.diagnostics)
        .unwrap_or_default()
}
/// Query implementation of [crate::db::SemanticGroup::extern_type_declaration_signature].
pub fn extern_type_declaration_generic_params(
    db: &dyn SemanticGroup,
    extern_type_id: ExternTypeId,
) -> Option<Vec<GenericParamId>> {
    Some(db.priv_extern_type_declaration_data(extern_type_id)?.generic_params)
}

// Computation.
/// Query implementation of [crate::db::SemanticGroup::priv_extern_type_declaration_data].
pub fn priv_extern_type_declaration_data(
    db: &dyn SemanticGroup,
    extern_type_id: ExternTypeId,
) -> Option<ExternTypeDeclarationData> {
    let module_id = extern_type_id.module(db.upcast());
    let mut diagnostics = SemanticDiagnostics::new(module_id);
    let module_data = db.module_data(module_id)?;
    let type_syntax = module_data.extern_types.get(&extern_type_id)?;
    let generic_params = semantic_generic_params(
        db,
        &mut diagnostics,
        module_id,
        &type_syntax.generic_params(db.upcast()),
    );
    Some(ExternTypeDeclarationData { diagnostics: diagnostics.diagnostics, generic_params })
}