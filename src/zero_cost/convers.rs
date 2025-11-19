use crate::{
    ast_specs::{StateMutability, Visibility},
    zero_cost::types::abstraction::{ZcStateMutability, ZcVisibility},
};

impl From<ZcStateMutability> for StateMutability {
    fn from(value: ZcStateMutability) -> Self {
        match value {
            ZcStateMutability::ZcPayable => StateMutability::Payable,
            ZcStateMutability::ZcPure => StateMutability::Pure,
            ZcStateMutability::ZcNonpayable => StateMutability::Nonpayable,
            ZcStateMutability::ZcView => StateMutability::View,
        }
    }
}

impl From<StateMutability> for ZcStateMutability {
    fn from(value: StateMutability) -> Self {
        match value {
            StateMutability::Payable => ZcStateMutability::ZcPayable,
            StateMutability::Pure => ZcStateMutability::ZcPure,
            StateMutability::Nonpayable => ZcStateMutability::ZcNonpayable,
            StateMutability::View => ZcStateMutability::ZcView,
        }
    }
}

impl From<ZcVisibility> for Visibility {
    fn from(value: ZcVisibility) -> Self {
        match value {
            ZcVisibility::ZcExternal => Visibility::External,
            ZcVisibility::ZcPublic => Visibility::Public,
            ZcVisibility::ZcInternal => Visibility::Internal,
            ZcVisibility::ZcPrivate => Visibility::Private,
        }
    }
}

impl From<Visibility> for ZcVisibility {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::External => ZcVisibility::ZcExternal,
            Visibility::Public => ZcVisibility::ZcPublic,
            Visibility::Internal => ZcVisibility::ZcInternal,
            Visibility::Private => ZcVisibility::ZcPrivate,
        }
    }
}
