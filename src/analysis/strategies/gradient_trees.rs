use std::error::Error;

use linfa_ensemble::{EnsembleLearner, EnsembleLearnerParams, EnsembleLearnerValidParams};
// use linfa_ensemble::{EnsembleLearner, EnsembleLearnerParams};
// use linfa::{Dataset};
use linfa_trees::{DecisionTree, DecisionTreeParams};
use linfa::{prelude::*, Label};
use linfa::traits::Fit;
use ndarray::{Array1, Array2};
use rand::prelude::*;
use rand::rngs::SmallRng;

use super::super::features::*;

pub fn prepare_dataset(features: &[featureset::DirectionClassificationFeatures]) -> (Array2<f32>, Array1<usize>) {
    let n_samples = features.len();
    let n_featues = 13;

    let mut feature_matrix = Array2::zeros((n_samples, n_featues));
    // let mut targets = Array1::zeros((n_samples,));
    let mut targets = Array1::from_elem((n_samples, ), 0);
    // targets.fill(String::default());

    for (i, feature) in features.iter().enumerate() {
        feature_matrix[[i, 0]] = feature.prev_close_to_high_ratio;
        feature_matrix[[i, 1]] = feature.prev_close_to_low_ratio;
        feature_matrix[[i, 2]] = feature.daily_return;
        feature_matrix[[i, 3]] = feature.volatility_5d;
        feature_matrix[[i, 4]] = feature.volatility_20d;
        feature_matrix[[i, 5]] = feature.volume_ratio_5d;
        feature_matrix[[i, 6]] = feature.volume_ratio_20d;
        feature_matrix[[i, 7]] = feature.rsi_14;
        feature_matrix[[i, 8]] = feature.sma_5;
        feature_matrix[[i, 9]] = feature.sma_20;
        feature_matrix[[i, 10]] = feature.macd_line;
        feature_matrix[[i, 11]] = feature.macd_signal;
        feature_matrix[[i, 12]] = feature.macd_histogram;
        
        targets[i] = feature.direction.to_usize();
    }
    
    (feature_matrix, targets)
}

pub fn train(train: &[featureset::DirectionClassificationFeatures]) -> Result<DecisionTree<f32, usize>, Box<dyn std::error::Error>> {
    let (features, targets) = prepare_dataset(train);

    let dataset = linfa::Dataset::new(features, targets);

    let model: DecisionTree<f32, usize>  = DecisionTree::params()
        .max_depth(Some(15))
        .split_quality(linfa_trees::SplitQuality::Entropy)
        .min_weight_split(5.0)
        .fit(&dataset)?;
    
    Ok(model)
}

pub fn train_ensemble(train: &[featureset::DirectionClassificationFeatures]) -> Result<EnsembleLearner<DecisionTree<f32, usize>>, Box<dyn Error>> {
    let (features, targets) = prepare_dataset(train);
    let mut rng = rand::rngs::SmallRng::seed_from_u64(69);
    let dataset = linfa::Dataset::new(features, targets);

    let model: EnsembleLearner<DecisionTree<f32, usize>> = EnsembleLearnerParams::new(DecisionTree::<f32, usize>::params()
        .max_depth(Some(6))
        .split_quality(linfa_trees::SplitQuality::Entropy)
        .min_weight_split(2.0)
        .min_weight_leaf(1.0)
    )
        .ensemble_size(200)
        .bootstrap_proportion(0.65)
        .fit(&dataset)?;


    Ok(model)
} 