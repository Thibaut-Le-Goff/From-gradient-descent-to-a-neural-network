////// Struct with the settings of the neural network //////
pub struct Network {
    pub network_struct: Vec<usize>,
    pub distrib: String,
       
    pub hidden_activ_fun: String,
    pub out_activ_fun: String,
}

pub struct DataSet {
    pub inputs: Vec<Vec<f32>>,
    pub observed_values: Vec<Vec<f32>>,
}

pub type FunType = Box<dyn Fn(&Vec<f32>)->Vec<f32>>;

pub mod net_init;
pub mod propagation;
pub mod back_prop;
pub mod calculations;