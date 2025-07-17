// use std::collections::HashMap;

// use hmmm::HMM;
// use ndarray::Array1;
// use rand::SeedableRng;

// use crate::data::types::TickerDataframe;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum PriceMovement {
//     StrongDown = 0,  // < -2%
//     Down = 1,        // -2% to -0.5%
//     Flat = 2,        // -0.5% to 0.5%
//     Up = 3,          // 0.5% to 2%
//     StrongUp = 4,    // > 2%
// }

// impl PriceMovement {
//     fn from_percentage_change(change: f32) -> Self {
//         match change {
//             x if x < -2.0 => PriceMovement::StrongDown,
//             x if x < -0.5 => PriceMovement::Down,
//             x if x < 0.5 => PriceMovement::Flat,
//             x if x < 2.0 => PriceMovement::Up,
//             _ => PriceMovement::StrongUp,
//         }
//     }
    
//     fn to_usize(self) -> usize {
//         self as usize
//     }
    
//     fn from_usize(value: usize) -> Option<Self> {
//         match value {
//             0 => Some(PriceMovement::StrongDown),
//             1 => Some(PriceMovement::Down),
//             2 => Some(PriceMovement::Flat),
//             3 => Some(PriceMovement::Up),
//             4 => Some(PriceMovement::StrongUp),
//             _ => None,
//         }
//     }
// }

// pub struct StockHMM {
//     hmm: HMM,
//     price_movements: Vec<PriceMovement>,
// }

// impl StockHMM {
//     pub fn new(ticker_data: &[TickerDataframe], n_states: usize) -> Self {
//         // Convert price data to percentage changes
//         let price_changes = Self::calculate_price_changes(ticker_data);
        
//         // Convert to discrete observations
//         let observations = Self::discretize_price_changes(&price_changes);
        
//         // Convert to ndarray for HMM training
//         let training_data = Array1::from(
//             observations.iter().map(|&pm| pm.to_usize()).collect::<Vec<_>>()
//         );
        
//         // Train the HMM
//         let mut rng = rand::rngs::StdRng::seed_from_u64(42);
//         let hmm = HMM::train(&training_data, n_states, 5, &mut rng);
        
//         Self {
//             hmm,
//             price_movements: observations,
//         }
//     }
//     pub fn calculate_price_changes(ticker_data: &[TickerDataframe]) -> Vec<f32> {
//         let mut changes = Vec::new();
        
//         for i in 1..ticker_data.len() {
//             let previous_close = ticker_data[i - 1].close;
//             let current_close = ticker_data[i].close;
//             let percentage_change = ((current_close - previous_close) / previous_close) * 100.0;
//             changes.push(percentage_change);
//         }

//         changes
//     }
    
//     pub fn discretize_price_changes(price_changes: &[f32]) -> Vec<PriceMovement> {
//         price_changes
//             .iter()
//             .map(|&change| PriceMovement::from_percentage_change(change))
//             .collect()
//     }
    
//     pub fn predict_next_movements(&self, n_predictions: usize) -> Vec<PriceMovement> {
//         let mut rng = rand::rngs::StdRng::seed_from_u64(123);
        
//         // Sample from the trained HMM
//         let predictions: Vec<usize> = self.hmm
//             .sampler(&mut rng)
//             .map(|sample| sample.y)
//             .take(n_predictions)
//             .collect();
        
//         predictions
//             .into_iter()
//             .filter_map(PriceMovement::from_usize)
//             .collect()
//     }
    
//     pub fn get_most_likely_sequence(&self, observations: &[PriceMovement]) -> Vec<usize> {
//         let obs_array = Array1::from_vec(
//             observations.iter().map(|&pm| pm.to_usize()).collect()
//         );
        
//         self.hmm.most_likely_sequence(&obs_array).to_vec()
//     }
    
//     pub fn analyze_patterns(&self) -> HashMap<PriceMovement, usize> {
//         let mut pattern_counts = HashMap::new();
        
//         for &movement in &self.price_movements {
//             *pattern_counts.entry(movement).or_insert(0) += 1;
//         }
        
//         pattern_counts
//     }
//     pub fn print_model_info(&self) {
//         println!("HMM Model Information:");
//         println!("Number of states: {}", self.hmm.n());
//         println!("Number of observations: {}", self.hmm.k());
//         println!("Training data length: {}", self.price_movements.len());
        
//         // Print transition matrix
//         println!("\nTransition Matrix A:");
//         for i in 0..self.hmm.n() {
//             for j in 0..self.hmm.n() {
//                 print!("{:.3} ", self.hmm.a[(i, j)]);
//             }
//             println!();
//         }
        
//         // Print observation matrix
//         println!("\nObservation Matrix B:");
//         let movement_names = ["StrongDown", "Down", "Flat", "Up", "StrongUp"];
//         for i in 0..self.hmm.n() {
//             println!("State {}: ", i);
//             for j in 0..self.hmm.k() {
//                 println!("  {}: {:.3}", movement_names[j], self.hmm.b[(i, j)]);
//             }
//         }
//     }
// }