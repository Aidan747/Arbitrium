// use linfa::{Dataset};
use linfa_trees::{DecisionTree, DecisionTreeParams};
use linfa::prelude::*;
use ndarray::{Array1, Array2};

use super::super::features::*;

fn prepare_training_data(features: &[featureset::DirectionClassificationFeatures]) -> (Array2<f32>, Array1<usize>) {
    let n_samples = features.len();
    let n_featues = 10;

    let mut feature_matrix = Array2::zeros((n_samples, n_featues));
    let mut targets = Array1::zeros((n_samples,));

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
        
        targets[i] = feature.direction.to_usize();
    }
    
    (feature_matrix, targets)
}

pub fn train(train: &[featureset::DirectionClassificationFeatures], test: &[featureset::DirectionClassificationFeatures]) -> Result<(), Box<dyn std::error::Error>> {
    let (features, targets) = prepare_training_data(train);

    let dataset = linfa::Dataset::new(features, targets);

    let (test_feats, test_targs) = prepare_training_data(test);

    let test_dataset = linfa::Dataset::new(test_feats, test_targs);

    // println!("{:#?}", dataset);

    // let model = DecisionTree::params()
    //     .max_depth(max_depth)

    // let params = DecisionTreeParams
    let model = linfa_trees::DecisionTree::params().fit(&dataset)?;

    let acuracy = model.predict(&test_dataset).confusion_matrix(&test_dataset).unwrap().accuracy();

    println!("{acuracy}");
    
    Ok(())
}