// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use super::{super::DirectEvaluatorInput, ApiEvaluatorError, API_CATEGORY};
use crate::{
    configuration::EvaluatorArgs,
    evaluator::{EvaluationResult, Evaluator},
    evaluators::EvaluatorType,
};
use anyhow::{anyhow, Result};
use aptos_rest_client::Client as AptosRestClient;
use aptos_sdk::crypto::HashValue;
use clap::Parser;
use poem_openapi::Object as PoemObject;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, Parser, PoemObject, Serialize)]
pub struct TransactionPresenceEvaluatorArgs {
    #[clap(long, default_value_t = 5)]
    pub transaction_fetch_delay_secs: u64,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TransactionPresenceEvaluator {
    args: TransactionPresenceEvaluatorArgs,
}

impl TransactionPresenceEvaluator {
    pub fn new(args: TransactionPresenceEvaluatorArgs) -> Self {
        Self { args }
    }
}

#[async_trait::async_trait]
impl Evaluator for TransactionPresenceEvaluator {
    type Input = DirectEvaluatorInput;
    type Error = ApiEvaluatorError;

    /// Assert that the target node can produce the same transaction that the
    /// baseline produced after a delay.
    async fn evaluate(&self, input: &Self::Input) -> Result<Vec<EvaluationResult>, Self::Error> {
        let baseline_client =
            AptosRestClient::new(input.baseline_node_information.node_address.get_api_url());

        let latest_baseline_transaction_hash = baseline_client
            .get_transactions(None, Some(1))
            .await
            .map_err(|e| {
                ApiEvaluatorError::EndpointError(
                    "/transactions".to_string(),
                    e.context(
                        "The baseline API endpoint failed to return a transaction".to_string(),
                    ),
                )
            })?
            .into_inner()
            .first()
            .ok_or_else(|| {
                ApiEvaluatorError::EndpointError(
                    "/transactions".to_string(),
                    anyhow!("The baseline API returned"),
                )
            })?
            .transaction_info()
            .map_err(|e| {
                ApiEvaluatorError::EndpointError(
                    "/transactions".to_string(),
                    e.context("The baseline returned a transaction with no info".to_string()),
                )
            })?
            .hash;

        tokio::time::sleep(Duration::from_secs(self.args.transaction_fetch_delay_secs)).await;

        let target_client = AptosRestClient::new(input.target_node_address.get_api_url());
        let evaluation = match target_client
            .get_transaction(HashValue::from(latest_baseline_transaction_hash))
            .await
        {
            Ok(_) => self.build_evaluation_result(
                "Target node produced recent transaction".to_string(),
                100,
                format!(
                    "We got the latest transaction from the baseline node ({}), waited {} \
                        seconds, and then asked your node to give us that transaction, and \
                        it did. Great! This implies that your node is keeping up with other \
                        nodes in the network.",
                    latest_baseline_transaction_hash, self.args.transaction_fetch_delay_secs,
                ),
            ),
            Err(e) => self.build_evaluation_result(
                "Target node failed to produce recent transaction".to_string(),
                50,
                format!(
                    "We got the latest transaction from the baseline node ({}), waited {} \
                        seconds, and then asked your node to give us that transaction, and \
                        it could not. This implies that your node is lagging behind the \
                        baseline by at least {} seconds. Error from your node: {}",
                    latest_baseline_transaction_hash,
                    self.args.transaction_fetch_delay_secs,
                    self.args.transaction_fetch_delay_secs,
                    e,
                ),
            ),
        };

        Ok(vec![evaluation])
    }

    fn get_category() -> String {
        API_CATEGORY.to_string()
    }

    fn get_name_suffix() -> String {
        "transaction_presence".to_string()
    }

    fn from_evaluator_args(evaluator_args: &EvaluatorArgs) -> Result<Self> {
        Ok(Self::new(evaluator_args.transaction_presence_args.clone()))
    }

    fn evaluator_type_from_evaluator_args(evaluator_args: &EvaluatorArgs) -> Result<EvaluatorType> {
        Ok(EvaluatorType::Api(Box::new(Self::from_evaluator_args(
            evaluator_args,
        )?)))
    }
}
