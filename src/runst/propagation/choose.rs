/////////////// Select the activation functions wanted ///////////

use crate::runst::Network;
use crate::runst::propagation::activ_fun::*;
use crate::runst::FunType;

pub fn activ_fun(hidden_activ_fun: &String, out_activ_fun: &String, activ_fun_list: &Vec<(FunType, &str)>) -> (usize, usize) {

    let mut hidden_activ_fun_i: usize = 0;
    let mut out_activ_fun_i: usize = 0;

    println!("hidden_activ_fun : {}", hidden_activ_fun);
    println!("out_activ_fun : {}", out_activ_fun);

    for i in 0..activ_fun_list.len() {
        if activ_fun_list[i].1 == hidden_activ_fun {
            hidden_activ_fun_i = i;
            println!("i : {}", i);
            println!("hidden_activ_fun_i : {}", hidden_activ_fun_i);
            //hidden_activ_fun_i = activ_fun_list[i].0;
            //break;
        }
        // not else if because the same fun can be use in the
        // hidden and out layers
        if activ_fun_list[i].1 == out_activ_fun {
            out_activ_fun_i = i;
            println!("i : {}", i);
            println!("out_activ_fun_i : {}", out_activ_fun_i);
            //out_activ_fun_i = activ_fun_list[i].0;
            //break;
        }
    }

    (hidden_activ_fun_i, out_activ_fun_i)
}