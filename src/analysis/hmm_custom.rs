use ndarray::{Array1, Array2, Axis};
use rand::{Rng, SeedableRng};
use rand_distr::{Distribution, Normal};
use std::f64::consts::PI;
use crate::data::types::TickerDataframe;

#[derive(Debug, Clone)]
pub struct GaussianEmission {
    pub mean: f64,
    pub variance: f64,
}

impl GaussianEmission {
    pub fn new(mean: f64, variance: f64) -> Self {
        Self { 
            mean, 
            variance: variance.max(1e-6) // Prevent zero variance
        }
    }
    
    /// Calculate probability density function value
    pub fn pdf(&self, x: f64) -> f64 {
        let coefficient = 1.0 / (2.0 * PI * self.variance).sqrt();
        let exponent = -0.5 * (x - self.mean).powi(2) / self.variance;
        coefficient * exponent.exp()
    }
    
    /// Sample from this Gaussian distribution
    pub fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        let normal = Normal::new(self.mean, self.variance.sqrt()).unwrap();
        normal.sample(rng)
    }
}

#[derive(Debug, Clone)]
pub struct ContinuousHMM {
    pub n_states: usize,
    pub transition_matrix: Array2<f64>,      // A[i][j] = P(state j | state i)
    pub initial_probabilities: Array1<f64>, // π[i] = P(initial state i)
    pub emission_params: Vec<GaussianEmission>, // Gaussian parameters for each state
}

impl ContinuousHMM {
    pub fn new(
        n_states: usize,
        transition_matrix: Array2<f64>,
        initial_probabilities: Array1<f64>,
        emission_params: Vec<GaussianEmission>,
    ) -> Self {
        assert_eq!(transition_matrix.shape(), &[n_states, n_states]);
        assert_eq!(initial_probabilities.len(), n_states);
        assert_eq!(emission_params.len(), n_states);
        
        Self {
            n_states,
            transition_matrix,
            initial_probabilities,
            emission_params,
        }
    }
    
    /// Initialize HMM with random parameters
    pub fn random_init(n_states: usize, seed: u64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    
        // Initialize transition matrix (row-stochastic)
        let mut transition_matrix = Array2::zeros((n_states, n_states));
        for i in 0..n_states {
            let mut row_sum = 0.0;
            for j in 0..n_states {
                let val: f64 = rng.random();
                transition_matrix[[i, j]] = val;
                row_sum += val;
            }
            // Normalize row
            for j in 0..n_states {
                transition_matrix[[i, j]] /= row_sum;
            }
        }
        
        // Initialize initial probabilities
        let mut initial_probabilities = Array1::zeros(n_states);
        let mut sum = 0.0;
        for i in 0..n_states {
            let val: f64 = rng.random();
            initial_probabilities[i] = val;
            sum += val;
        }
        initial_probabilities /= sum;
        
        // Initialize emission parameters with different means
        let mut emission_params = Vec::new();
        for i in 0..n_states {
            let mean = (i as f64 - (n_states as f64 - 1.0) / 2.0) * 2.0; // Spread means
            let variance = 1.0 + rng.random::<f64>(); // Random variance between 1 and 2
            emission_params.push(GaussianEmission::new(mean, variance));
        }
        
        Self::new(n_states, transition_matrix, initial_probabilities, emission_params)
    }
    
    /// Train HMM using Baum-Welch algorithm
    pub fn train_baum_welch(
        observations: &[f64],
        n_states: usize,
        max_iterations: usize,
        tolerance: f64,
        seed: u64,
    ) -> Self {
        let mut hmm = Self::random_init(n_states, seed);
        let mut prev_log_likelihood = f64::NEG_INFINITY;
        
        println!("Starting Baum-Welch training with {} observations, {} states", 
                observations.len(), n_states);
        
        for iteration in 0..max_iterations {
            // E-step: Forward-Backward algorithm
            let (alpha, log_likelihood) = hmm.forward_algorithm(observations);
            let beta = hmm.backward_algorithm(observations);
            
            // Calculate gamma (state probabilities) and xi (transition probabilities)
            let gamma = hmm.calculate_gamma(&alpha, &beta);
            let xi = hmm.calculate_xi(&alpha, &beta, observations);
            
            // M-step: Update parameters
            hmm.update_parameters(&gamma, &xi, observations);
            
            // Check convergence
            let improvement = log_likelihood - prev_log_likelihood;
            println!("Iteration {}: Log-likelihood = {:.6}, Improvement = {:.6}", 
                    iteration + 1, log_likelihood, improvement);
            
            if improvement.abs() < tolerance {
                println!("Converged after {} iterations", iteration + 1);
                break;
            }
            
            prev_log_likelihood = log_likelihood;
        }
        
        hmm
    }
    
    /// Forward algorithm: compute α[t][i] = P(o1...ot, qt = i | λ)
    pub fn forward_algorithm(&self, observations: &[f64]) -> (Array2<f64>, f64) {
        let t_max = observations.len();
        let mut alpha = Array2::zeros((t_max, self.n_states));
        let mut scaling_factors = Array1::zeros(t_max);
        
        // Initialize α[0][i]
        for i in 0..self.n_states {
            alpha[[0, i]] = self.initial_probabilities[i] 
                          * self.emission_params[i].pdf(observations[0]);
        }
        
        // Scale α[0]
        scaling_factors[0] = alpha.row(0).sum();
        if scaling_factors[0] > 0.0 {
            alpha.row_mut(0).mapv_inplace(|x| x / scaling_factors[0]);
        }
        
        // Forward pass
        for t in 1..t_max {
            for j in 0..self.n_states {
                let mut sum = 0.0;
                for i in 0..self.n_states {
                    sum += alpha[[t - 1, i]] * self.transition_matrix[[i, j]];
                }
                alpha[[t, j]] = sum * self.emission_params[j].pdf(observations[t]);
            }
            
            // Scale α[t]
            scaling_factors[t] = alpha.row(t).sum();
            if scaling_factors[t] > 0.0 {
                alpha.row_mut(t).mapv_inplace(|x| x / scaling_factors[t]);
            }
        }
        
        // Calculate log-likelihood
        let log_likelihood = scaling_factors.iter()
            .filter(|&&x| x > 0.0)
            .map(|&x| x.ln())
            .sum();
        
        (alpha, log_likelihood)
    }
    
    /// Backward algorithm: compute β[t][i] = P(ot+1...oT | qt = i, λ)
    pub fn backward_algorithm(&self, observations: &[f64]) -> Array2<f64> {
        let t_max = observations.len();
        let mut beta = Array2::zeros((t_max, self.n_states));
        
        // Initialize β[T-1][i] = 1
        for i in 0..self.n_states {
            beta[[t_max - 1, i]] = 1.0;
        }
        
        // Backward pass
        for t in (0..t_max - 1).rev() {
            for i in 0..self.n_states {
                let mut sum = 0.0;
                for j in 0..self.n_states {
                    sum += self.transition_matrix[[i, j]]
                         * self.emission_params[j].pdf(observations[t + 1])
                         * beta[[t + 1, j]];
                }
                beta[[t, i]] = sum;
            }
            
            // Apply same scaling as forward algorithm
            let scale_factor = beta.row(t).sum();
            if scale_factor > 0.0 {
                beta.row_mut(t).mapv_inplace(|x| x / scale_factor);
            }
        }
        
        beta
    }
    
    /// Calculate γ[t][i] = P(qt = i | O, λ)
    fn calculate_gamma(&self, alpha: &Array2<f64>, beta: &Array2<f64>) -> Array2<f64> {
        let (t_max, n_states) = alpha.dim();
        let mut gamma = Array2::zeros((t_max, n_states));
        
        for t in 0..t_max {
            let mut row_sum = 0.0;
            for i in 0..n_states {
                gamma[[t, i]] = alpha[[t, i]] * beta[[t, i]];
                row_sum += gamma[[t, i]];
            }
            
            // Normalize
            if row_sum > 0.0 {
                for i in 0..n_states {
                    gamma[[t, i]] /= row_sum;
                }
            }
        }
        
        gamma
    }
    
    /// Calculate ξ[t][i][j] = P(qt = i, qt+1 = j | O, λ)
    fn calculate_xi(
        &self,
        alpha: &Array2<f64>,
        beta: &Array2<f64>,
        observations: &[f64],
    ) -> Array2<f64> {
        let t_max = observations.len();
        let mut xi_sum = Array2::zeros((self.n_states, self.n_states));
        
        for t in 0..t_max - 1 {
            let mut normalizer = 0.0;
            let mut xi_t = Array2::zeros((self.n_states, self.n_states));
            
            // Calculate ξ[t][i][j] for all i, j
            for i in 0..self.n_states {
                for j in 0..self.n_states {
                    xi_t[[i, j]] = alpha[[t, i]]
                                 * self.transition_matrix[[i, j]]
                                 * self.emission_params[j].pdf(observations[t + 1])
                                 * beta[[t + 1, j]];
                    normalizer += xi_t[[i, j]];
                }
            }
            
            // Normalize and accumulate
            if normalizer > 0.0 {
                for i in 0..self.n_states {
                    for j in 0..self.n_states {
                        xi_sum[[i, j]] += xi_t[[i, j]] / normalizer;
                    }
                }
            }
        }
        
        xi_sum
    }
    
    /// M-step: Update HMM parameters
    fn update_parameters(
        &mut self,
        gamma: &Array2<f64>,
        xi: &Array2<f64>,
        observations: &[f64],
    ) {
        let t_max = observations.len();
        
        // Update initial probabilities: π[i] = γ[0][i]
        for i in 0..self.n_states {
            self.initial_probabilities[i] = gamma[[0, i]];
        }
        
        // Update transition probabilities: A[i][j] = Σ_t ξ[t][i][j] / Σ_t γ[t][i]
        for i in 0..self.n_states {
            let denominator: f64 = (0..t_max - 1).map(|t| gamma[[t, i]]).sum();
            
            if denominator > 1e-10 {
                for j in 0..self.n_states {
                    self.transition_matrix[[i, j]] = xi[[i, j]] / denominator;
                }
            }
        }
        
        // Update emission parameters (Gaussian mean and variance)
        for i in 0..self.n_states {
            let weight_sum: f64 = (0..t_max).map(|t| gamma[[t, i]]).sum();
            
            if weight_sum > 1e-10 {
                // Update mean: μ[i] = Σ_t γ[t][i] * o[t] / Σ_t γ[t][i]
                let weighted_obs_sum: f64 = (0..t_max)
                    .map(|t| gamma[[t, i]] * observations[t])
                    .sum();
                let new_mean = weighted_obs_sum / weight_sum;
                
                // Update variance: σ²[i] = Σ_t γ[t][i] * (o[t] - μ[i])² / Σ_t γ[t][i]
                let weighted_var_sum: f64 = (0..t_max)
                    .map(|t| gamma[[t, i]] * (observations[t] - new_mean).powi(2))
                    .sum();
                let new_variance = (weighted_var_sum / weight_sum).max(1e-6);
                
                self.emission_params[i] = GaussianEmission::new(new_mean, new_variance);
            }
        }
    }
    
    /// Viterbi algorithm: find most likely state sequence
    pub fn viterbi(&self, observations: &[f64]) -> Vec<usize> {
        let t_max = observations.len();
        let mut delta = Array2::zeros((t_max, self.n_states));
        let mut psi = Array2::zeros((t_max, self.n_states));
        
        // Initialize
        for i in 0..self.n_states {
            delta[[0, i]] = self.initial_probabilities[i].ln()
                          + self.emission_params[i].pdf(observations[0]).ln();
        }
        
        // Forward pass
        for t in 1..t_max {
            for j in 0..self.n_states {
                let mut max_val = f64::NEG_INFINITY;
                let mut max_idx = 0;
                
                for i in 0..self.n_states {
                    let val = delta[[t - 1, i]] + self.transition_matrix[[i, j]].ln();
                    if val > max_val {
                        max_val = val;
                        max_idx = i;
                    }
                }
                
                delta[[t, j]] = max_val + self.emission_params[j].pdf(observations[t]).ln();
                psi[[t, j]] = max_idx as f64;
            }
        }
        
        // Backward pass
        let mut path = vec![0; t_max];
        
        // Find best final state
        let mut max_val = f64::NEG_INFINITY;
        for i in 0..self.n_states {
            if delta[[t_max - 1, i]] > max_val {
                max_val = delta[[t_max - 1, i]];
                path[t_max - 1] = i;
            }
        }
        
        // Trace back
        for t in (0..t_max - 1).rev() {
            path[t] = psi[[t + 1, path[t + 1]]] as usize;
        }
        
        path
    }
    
    /// Generate sequence of observations
    pub fn generate_sequence(&self, length: usize, seed: u64) -> (Vec<f64>, Vec<usize>) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut observations = Vec::new();
        let mut states = Vec::new();
        
        // Sample initial state
        let cumulative_initial: Vec<f64> = self.initial_probabilities.iter()
            .scan(0.0, |acc, &x| { *acc += x; Some(*acc) })
            .collect();
        
        let mut current_state = 0;
        let random_val: f64 = rng.random();
        for (i, &cum_prob) in cumulative_initial.iter().enumerate() {
            if random_val <= cum_prob {
                current_state = i;
                break;
            }
        }
        
        // Generate sequence
        for _ in 0..length {
            states.push(current_state);
            
            // Sample observation from current state
            let obs = self.emission_params[current_state].sample(&mut rng);
            observations.push(obs);
            
            // Transition to next state
            let cumulative_trans: Vec<f64> = (0..self.n_states)
                .scan(0.0, |acc, j| {
                    *acc += self.transition_matrix[[current_state, j]];
                    Some(*acc)
                })
                .collect();
            
            let random_val: f64 = rng.random();
            for (j, &cum_prob) in cumulative_trans.iter().enumerate() {
                if random_val <= cum_prob {
                    current_state = j;
                    break;
                }
            }
        }
        
        (observations, states)
    }
    
    /// Calculate log-likelihood of observation sequence
    pub fn log_likelihood(&self, observations: &[f64]) -> f64 {
        let (_, log_likelihood) = self.forward_algorithm(observations);
        log_likelihood
    }
    
    /// Print model parameters
    pub fn print_model(&self) {
        println!("HMM Model Parameters:");
        println!("Number of states: {}", self.n_states);
        
        println!("\nInitial probabilities:");
        for (i, &prob) in self.initial_probabilities.iter().enumerate() {
            println!("  π[{}] = {:.4}", i, prob);
        }
        
        println!("\nTransition matrix:");
        for i in 0..self.n_states {
            print!("  ");
            for j in 0..self.n_states {
                print!("A[{},{}]={:.3} ", i, j, self.transition_matrix[[i, j]]);
            }
            println!();
        }
        
        println!("\nEmission parameters (Gaussian):");
        for (i, params) in self.emission_params.iter().enumerate() {
            println!("  State {}: μ={:.3}, σ²={:.3}", i, params.mean, params.variance);
        }
    }
}

// Integration with your ticker data
impl ContinuousHMM {
    /// Train HMM on stock price data
    pub fn train_on_ticker_data(
        ticker_data: &[TickerDataframe],
        n_states: usize,
        max_iterations: usize,
    ) -> Self {
        // Extract price changes as continuous observations
        let observations = Self::extract_price_changes(ticker_data);
        
        println!("Training HMM on {} price change observations", observations.len());
        
        Self::train_baum_welch(&observations, n_states, max_iterations, 1e-6, 42)
    }
    
    fn extract_price_changes(ticker_data: &[TickerDataframe]) -> Vec<f64> {
        let mut changes = Vec::new();
        
        for i in 1..ticker_data.len() {
            let prev_close = ticker_data[i - 1].close as f64;
            let curr_close = ticker_data[i].close as f64;
            let pct_change = ((curr_close - prev_close) / prev_close) * 100.0;
            changes.push(pct_change);
        }
        
        changes
    }
    
    /// Predict future price movements
    pub fn predict_price_movements(&self, n_predictions: usize) -> Vec<f64> {
        let (predictions, _) = self.generate_sequence(n_predictions, 123);
        predictions
    }
    
    // Analyze market regimes in historical data
    // pub fn analyze_market_regimes(&self, ticker_data: &[TickerDataframe]) -> Vec<(usize, String)> {
    //     let observations = Self::extract_price_changes(ticker_data);   
    //     let state_sequence = self.viterbi(&observations);
        
    //     let regime_names = match self.n_states {
    //         3 => vec!["Bearish", "Neutral", "Bullish"],
    //         2 => vec!["Bear Market", "Bull Market"],
    //         _ => (0..self.n_states).map(|i| format!("Regime {}", i)).collect(),
    //     };
        
    //     state_sequence.into_iter()
    //         .map(|state| (state, regime_names[state].to_string()))
    //         .collect()
    // }
}