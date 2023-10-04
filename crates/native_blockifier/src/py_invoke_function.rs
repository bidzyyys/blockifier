use std::convert::TryFrom;
use std::sync::Arc;

use blockifier::transaction::transaction_types::TransactionType;
use blockifier::transaction::transactions::InvokeTransaction;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use starknet_api::core::{ContractAddress, EntryPointSelector, Nonce};
use starknet_api::data_availability::DataAvailabilityMode;
use starknet_api::hash::StarkFelt;
use starknet_api::transaction::{
    AccountDeploymentData, Calldata, Fee, InvokeTransactionV0, InvokeTransactionV1,
    InvokeTransactionV3, PaymasterData, ResourceBoundsMapping, Tip, TransactionHash,
    TransactionSignature, TransactionVersion,
};

use crate::errors::{NativeBlockifierInputError, NativeBlockifierResult};
use crate::py_transaction::{PyDataAvailabilityMode, PyResourceBoundsMapping};
use crate::py_utils::{from_py_felts, py_attr, PyFelt};

#[derive(FromPyObject)]
pub struct PyInvokeTransactionV0 {
    pub max_fee: u128,
    pub signature: Vec<PyFelt>,
    pub contract_address: PyFelt,
    pub entry_point_selector: PyFelt,
    pub calldata: Vec<PyFelt>,
    pub hash_value: PyFelt,
}

impl TryFrom<PyInvokeTransactionV0> for InvokeTransactionV0 {
    type Error = NativeBlockifierInputError;
    fn try_from(tx: PyInvokeTransactionV0) -> Result<Self, Self::Error> {
        Ok(Self {
            max_fee: Fee(tx.max_fee),
            signature: TransactionSignature(from_py_felts(tx.signature)),
            contract_address: ContractAddress::try_from(tx.contract_address.0)?,
            entry_point_selector: EntryPointSelector(tx.entry_point_selector.0),
            calldata: Calldata(Arc::from(from_py_felts(tx.calldata))),
        })
    }
}

#[derive(FromPyObject)]
pub struct PyInvokeTransactionV1 {
    pub max_fee: u128,
    pub signature: Vec<PyFelt>,
    pub nonce: PyFelt,
    pub sender_address: PyFelt,
    pub calldata: Vec<PyFelt>,
    pub hash_value: PyFelt,
}

impl TryFrom<PyInvokeTransactionV1> for InvokeTransactionV1 {
    type Error = NativeBlockifierInputError;
    fn try_from(tx: PyInvokeTransactionV1) -> Result<Self, Self::Error> {
        Ok(Self {
            max_fee: Fee(tx.max_fee),
            signature: TransactionSignature(from_py_felts(tx.signature)),
            nonce: Nonce(tx.nonce.0),
            sender_address: ContractAddress::try_from(tx.sender_address.0)?,
            calldata: Calldata(Arc::from(from_py_felts(tx.calldata))),
        })
    }
}

#[derive(FromPyObject)]
pub struct PyInvokeTransactionV3 {
    pub resource_bounds: PyResourceBoundsMapping,
    pub tip: u64,
    pub signature: Vec<PyFelt>,
    pub nonce: PyFelt,
    pub sender_address: PyFelt,
    pub calldata: Vec<PyFelt>,
    pub nonce_data_availability_mode: PyDataAvailabilityMode,
    pub fee_data_availability_mode: PyDataAvailabilityMode,
    pub paymaster_data: Vec<PyFelt>,
    pub account_deployment_data: Vec<PyFelt>,
    pub hash_value: PyFelt,
}

impl TryFrom<PyInvokeTransactionV3> for InvokeTransactionV3 {
    type Error = NativeBlockifierInputError;
    fn try_from(tx: PyInvokeTransactionV3) -> Result<Self, Self::Error> {
        Ok(Self {
            resource_bounds: ResourceBoundsMapping::from(tx.resource_bounds),
            tip: Tip(tx.tip),
            signature: TransactionSignature(from_py_felts(tx.signature)),
            nonce: Nonce(tx.nonce.0),
            sender_address: ContractAddress::try_from(tx.sender_address.0)?,
            calldata: Calldata(Arc::from(from_py_felts(tx.calldata))),
            nonce_data_availability_mode: DataAvailabilityMode::from(
                tx.nonce_data_availability_mode,
            ),
            fee_data_availability_mode: DataAvailabilityMode::from(tx.fee_data_availability_mode),
            paymaster_data: PaymasterData(from_py_felts(tx.paymaster_data)),
            account_deployment_data: AccountDeploymentData(from_py_felts(
                tx.account_deployment_data,
            )),
        })
    }
}

// Transactions creation.

pub enum PyInvoke {
    V0(PyInvokeTransactionV0),
    V1(PyInvokeTransactionV1),
    V3(PyInvokeTransactionV3),
}

impl PyInvoke {
    fn hash_value(&self) -> StarkFelt {
        let py_hash_value = match self {
            PyInvoke::V0(tx) => tx.hash_value,
            PyInvoke::V1(tx) => tx.hash_value,
            PyInvoke::V3(tx) => tx.hash_value,
        };

        py_hash_value.0
    }
}

impl FromPyObject<'_> for PyInvoke {
    fn extract(py_tx: &PyAny) -> PyResult<Self> {
        let py_version: PyFelt = py_tx.getattr("version")?.extract()?;
        let version = TransactionVersion(py_version.0);

        match version {
            TransactionVersion::ZERO => Ok(PyInvoke::V0(py_tx.extract()?)),
            TransactionVersion::ONE => Ok(PyInvoke::V1(py_tx.extract()?)),
            TransactionVersion::THREE => Ok(PyInvoke::V3(py_tx.extract()?)),
            _ => Err(PyValueError::new_err(format!("Invalid transaction version: {version:?}"))),
        }
    }
}

impl TryFrom<PyInvoke> for InvokeTransaction {
    type Error = NativeBlockifierInputError;
    fn try_from(py_tx: PyInvoke) -> Result<Self, Self::Error> {
        let tx_hash = TransactionHash(py_tx.hash_value());
        let tx = match py_tx {
            PyInvoke::V0(tx) => {
                starknet_api::transaction::InvokeTransaction::V0(InvokeTransactionV0::try_from(tx)?)
            }
            PyInvoke::V1(tx) => {
                starknet_api::transaction::InvokeTransaction::V1(InvokeTransactionV1::try_from(tx)?)
            }
            PyInvoke::V3(tx) => {
                starknet_api::transaction::InvokeTransaction::V3(InvokeTransactionV3::try_from(tx)?)
            }
        };

        Ok(InvokeTransaction { tx, tx_hash })
    }
}

pub fn py_invoke_function(py_tx: &PyAny) -> NativeBlockifierResult<InvokeTransaction> {
    let version = usize::try_from(py_attr::<PyFelt>(py_tx, "version")?.0)?;
    let tx = match version {
        0 => {
            let py_invoke_tx: PyInvokeTransactionV0 = py_tx.extract()?;
            let invoke_tx = InvokeTransactionV0::try_from(py_invoke_tx)?;
            Ok(starknet_api::transaction::InvokeTransaction::V0(invoke_tx))
        }
        1 => {
            let py_invoke_tx: PyInvokeTransactionV1 = py_tx.extract()?;
            let invoke_tx = InvokeTransactionV1::try_from(py_invoke_tx)?;
            Ok(starknet_api::transaction::InvokeTransaction::V1(invoke_tx))
        }
        3 => {
            let py_invoke_tx: PyInvokeTransactionV3 = py_tx.extract()?;
            let invoke_tx = InvokeTransactionV3::try_from(py_invoke_tx)?;
            Ok(starknet_api::transaction::InvokeTransaction::V3(invoke_tx))
        }
        _ => Err(NativeBlockifierInputError::UnsupportedTransactionVersion {
            tx_type: TransactionType::InvokeFunction,
            version,
        }),
    }?;

    let tx_hash = TransactionHash(py_attr::<PyFelt>(py_tx, "hash_value")?.0);
    Ok(InvokeTransaction { tx, tx_hash })
}
