use crate::backtracking::backtrack;
use crate::problem::Sudoku;
const NVAR:usize = 4;
const BLOCKSIZE:u8 =2;

mod problem{

    use std::collections::HashSet;
    use std::io;
    use std::io:: Write;
    use crate::NVAR;

    #[derive(Debug)]
    pub struct Sudoku {
        assigned_variables: HashSet<(u8, u8)>,
        inferences: HashSet<(u8, u8)>,
        unassigned_variables: HashSet<(u8, u8)>,
        domains: [[Vec<u8>;NVAR];NVAR],
    }
    impl Clone for Sudoku{
        fn clone(&self) -> Self {
            Self{assigned_variables:self.assigned_variables.clone(),inferences:self.inferences.clone(),
            unassigned_variables:self.unassigned_variables.clone(),domains:self.domains.clone()}
        }
    }

    impl Sudoku{
        pub fn new()->Self{
            let default_vector:Vec<u8>= (1..(NVAR+1) as u8).collect();
            let mut variables:HashSet<(u8,u8)> = HashSet::new();
            let mut domains: [[Vec<u8>; NVAR]; NVAR] = Default::default();
            for i in 0..NVAR{
                for j in 0..NVAR{
                    variables.insert((i as u8, j as u8));
                    domains[i][j]=default_vector.to_owned();
                }
            }
            return Sudoku{assigned_variables:HashSet::new(),inferences:HashSet::new(),unassigned_variables:variables,domains };
        }

        pub fn set_variable(&mut self, var:(u8, u8), value:u8){
            println!("setting variable {:?} to {},",var,value);
            self.unassigned_variables.remove(&var);
            self.inferences.remove(&var);
            self.assigned_variables.insert(var);
            self.domains[var.0 as usize][var.1 as usize].clear();
            self.domains[var.0 as usize][var.1 as usize].push(value);
            if self.domains[var.0 as usize][var.1 as usize].len()>1{
                panic!("there is an assignment with more than one value!");
            }
            for element in self.domains[var.0 as usize][var.1 as usize].to_owned(){
                println!("the variable has now value {}",element);
            }

        }

        pub fn add_inferences(&mut self,inferences: Vec<((u8,u8),Vec<u8>)>){
            for inference in inferences{
                if inference.1.len()!=1 {
                    if !self.get_inferences().contains(&inference.0) {
                        self.inferences.insert(inference.0);
                        self.unassigned_variables.remove(&inference.0);
                    }
                    self.domains[inference.0.0 as usize][inference.0.1 as usize].clear();
                    for value in inference.1 {
                        self.domains[inference.0.0 as usize][inference.0.1 as usize].push(value);
                    }
                }
                else{
                    self.set_variable(inference.0,inference.1[0]);
                }
            }
        }

        pub fn clear_inferences(&mut self){
            for variable in self.inferences.iter(){
                if self.domains[variable.0 as usize][variable.1 as usize].len()==1 {
                    panic!("what you are clearing is not an inference!");
                }
                self.domains[variable.0 as usize][variable.1 as usize].clear();
                self.unassigned_variables.insert(*variable);
                for i in 1..(NVAR+1){
                    self.domains[variable.0 as usize][variable.1 as usize].push(i as u8);
                }
            }
            self.inferences.clear();
        }

        pub fn remove_assignment(&mut self,var: (u8,u8), bad_value:u8){
            self.domains[var.0 as usize][ var.1 as usize].clear();
            self.assigned_variables.remove(&var);
            self.unassigned_variables.insert(var);

            for i in 1..(NVAR+1){
                if i != bad_value as usize {
                    self.domains[var.0 as usize][var.1 as usize].push(i as u8);
                }
            }
        }

        pub fn get_random_unassigned_variable(&self) -> (u8, u8) {
            return if self.inferences.is_empty() {
                println!("picking an unassigned variable");
                self.unassigned_variables.iter().last().unwrap().clone()
            } else {
                println!("picking an inference");
                self.inferences.iter().last().unwrap().clone()
            }
        }

        pub fn get_assignments(&self) -> HashSet<(u8,u8)>{
            return self.assigned_variables.clone();
        }

        pub fn get_unassigned_variables(&self)-> HashSet<(u8,u8)>{
            return self.unassigned_variables.clone();
        }

        pub fn get_inferences(&self)->HashSet<(u8,u8)>{
            return self.inferences.clone();
        }

        pub fn get_domain(&self, variable:(u8,u8))->Vec<u8>{
            return Vec::clone(&self.domains[variable.0 as usize][variable.1 as usize]);
        }


        pub fn is_complete(&self)->bool{

             return self.assigned_variables.len()==NVAR*NVAR;
        }


        pub fn fill(&mut self){
            let mut input=String::new();
            let mut value:u8;
            let mut x:u8;
            let mut y:u8;
            loop{
                println!("insert value or press q to quit");
                io::stdin().read_line(&mut input).expect("panic");

                if input.contains("q"){
                    break;
                }
                else{
                    value= input.trim().parse().unwrap();
                }
                input.clear();
                println!("insert value's x coordinate");
                io::stdin().read_line(&mut input).expect("panic");
                x= input.trim().parse().unwrap();
                input.clear();
                println!("insert value's y coordinate");
                io::stdin().read_line(&mut input).expect("panic");
                y= input.trim().parse().unwrap();
                input.clear();

                self.set_variable((x,y),value);
            }
        }

        pub fn print(&self){
            for i in 0..NVAR{
                for j in 0..NVAR{
                    if self.domains[i][j].len()==0{
                        print!("e ");
                        io::stdout().flush().unwrap();
                    }
                    else if self.assigned_variables.contains(&(i as u8,j as u8)){
                        print!("{} ",self.domains[i][j][0]);
                        io::stdout().flush().unwrap();
                    }
                    else{
                        print!("b ");
                        io::stdout().flush().unwrap();
                    }

                }
                print!("\n");
            }
            print!("\n");
        }
    }

}

mod backtracking{
//    use std::time::Duration;
//    use std::thread::sleep;
    use crate::{BLOCKSIZE, NVAR};

    use crate::problem::Sudoku;

    fn select_unassigned_variable(p:&Sudoku)->(u8,u8){
        //TODO write a better function
        return p.get_random_unassigned_variable();
    }

    fn order_domain_values(p:&Sudoku,var: (u8,u8))->Vec<u8>{
        //TODO write a better function
        return p.get_domain(var);
    }

    fn consistency_helper(p:&Sudoku, start_domain:Vec<u8>,variable:(u8,u8))->Vec<u8>{
        let mut new_domain:Vec<u8>= Vec::new();
        for value in start_domain{
            if value != p.get_domain(variable)[0]{
                new_domain.push(value);
            }
        }
        return new_domain;
    }

    fn is_consistent(p:& Sudoku,new_variable:(u8,u8),value:u8)->bool{
        let mut domain=p.get_domain(new_variable);
        for assigned_variable in p.get_assignments(){
            if new_variable.0==assigned_variable.0||new_variable.1==assigned_variable.1||
                (new_variable.0/BLOCKSIZE==assigned_variable.0/BLOCKSIZE && new_variable.1/BLOCKSIZE==assigned_variable.1/BLOCKSIZE){
                domain = consistency_helper(p,domain,assigned_variable);
            }
        }
        return domain.contains(&value);
    }

    fn inference(p:&Sudoku, new_variable:(u8,u8))->Result<Vec<((u8,u8),Vec<u8>)>,String>{
        let mut inferences:Vec<((u8,u8),Vec<u8>)> = Vec::new();
        let mut inconsistency_found = false;
        let mut start_domain:Vec<u8>;
        let mut new_domain:Vec<u8>;
        for unassigned_variable in p.get_unassigned_variables(){
            if new_variable.0==unassigned_variable.0||new_variable.1==unassigned_variable.1||
                (new_variable.0/BLOCKSIZE==unassigned_variable.0/BLOCKSIZE && new_variable.1/BLOCKSIZE==unassigned_variable.1/BLOCKSIZE){
                println!("forward checking variable {:?}",unassigned_variable);
                start_domain = p.get_domain(unassigned_variable);
                new_domain=consistency_helper(p,start_domain,new_variable);
                if new_domain.is_empty(){
                    inconsistency_found= true;
                }
                inferences.push((unassigned_variable,new_domain));
            }
        }
        for old_inference in p.get_inferences(){
            if new_variable.0==old_inference.0||new_variable.1==old_inference.1||
                (new_variable.0/BLOCKSIZE==old_inference.0/BLOCKSIZE && new_variable.1/BLOCKSIZE==old_inference.1/BLOCKSIZE){
                println!("forward checking inference {:?}",old_inference);
                start_domain = p.get_domain(old_inference);
                new_domain=consistency_helper(p,start_domain,new_variable);
                if new_domain.is_empty(){
                    inconsistency_found= true;
                }
                inferences.push((old_inference,new_domain));
            }
        }

        return if !inconsistency_found {
            Ok(inferences)
        } else {
            println!("bla  ");
            p.print();
            Err("Failure".to_string())
        }

    }


    pub fn backtrack(mut p: Sudoku,step:u32) -> Result<Sudoku,String>{
        //qlet time = Duration::from_millis(1000);
        if p.get_assignments().len()+p.get_inferences().len()+p.get_unassigned_variables().len()!=NVAR*NVAR{
            panic!("I have got a mess")
        }

        //sleep(time);
        println!("step number {}",step);
        p.print();
        println!("problem state is {:?},",p);
        println!(" ");
        if p.is_complete(){
            return Ok(p);
        }
        let var = select_unassigned_variable(&p);
        for value  in order_domain_values(&p, var) {
            if is_consistent(&p, var, value) {
                p.set_variable(var, value);
                let inferences = inference(&p, var);
                let go = match inferences{
                    Ok(..)=>true,
                    Err(..)=>false
                };
                if go {
                    p.add_inferences(inferences.unwrap() );
                    if p.get_assignments().len()+p.get_inferences().len()+p.get_unassigned_variables().len()!=NVAR*NVAR{
                        panic!("I have made a mess")
                    }
                    let result = backtrack(p.clone(),step+1);

                    //

                    let exit =match result {
                        Ok(..) => true,
                        Err(..) => false
                    };
                    if exit {
                        return result;
                    };
                    p.clear_inferences();
                }
                p.remove_assignment(var,value);

            }

        }
        println!("failure");
        return Err("Failure0".to_string());

    }
}





fn main() {
    let mut s =Sudoku::new();
    s.fill();
    println!("{:?}", backtrack(s,0).unwrap());

}
