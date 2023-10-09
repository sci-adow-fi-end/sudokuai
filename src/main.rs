
mod problem{

    use std::collections::HashSet;
    pub struct Sudoku {
        assigned_variables: HashSet<(u8, u8)>,
        inferences: HashSet<(u8, u8)>,
        unassigned_variables: HashSet<(u8, u8)>,
        domains: [[Vec<u8>;9];9],
    }
    impl Sudoku{
        pub fn set_variable(mut self, var:(u8, u8), value:u8){
            self.assigned_variables.insert(var);
            self.domains[var.0 as usize][var.1 as usize].clear();
            self.domains[var.0 as usize][var.1 as usize].push(value);
        }

        pub fn add_inferences(&mut self,inferences: Vec<((u8,u8),Vec<u8>)>){
            for inference in inferences{
                self.inferences.insert(inference.0);
                self.domains[inference.0.0 as usize][inference.0.0 as usize].clear();
                for value in inference.1{
                    self.domains[inference.0.0 as usize][inference.0.0 as usize].push(value);
                }
            }
        }

        pub fn clear_inferences(&mut self){
            for variable in self.inferences.iter(){
                self.domains[variable.0 as usize][variable.1 as usize].clear();
                for i in 1..9{
                    self.domains[variable.0 as usize][variable.1 as usize].push(i);
                }
            }
            self.inferences.clear();
        }

        pub fn remove_assignment(&mut self,var: (u8,u8), bad_value:u8){
            self.domains[var.0 as usize][ var.1 as usize].clear();

            for i in 1..9{
                if i != bad_value {
                    self.domains[var.0 as usize][var.1 as usize].push(i);
                }
            }
        }

        pub fn get_random_unassigned_variable(&self) -> (u8, u8) {
            return self.unassigned_variables.iter().last().unwrap().clone();
        }

        pub fn get_assignments(&self) -> HashSet<(u8,u8)>{
            return self.assigned_variables.clone();
        }

        pub fn get_unassigned_variables(&self)-> HashSet<(u8,u8)>{
            return self.unassigned_variables.clone();
        }

        pub fn get_domain(&self, variable:(u8,u8))->Vec<u8>{
            return Vec::clone(&self.domains[variable.0 as usize][variable.1 as usize]);
        }


        pub fn is_complete(&self)->bool{

             return self.unassigned_variables.is_empty();
        }


        pub fn fill(&self){
            let mut input=String::new();
            let mut value:u8;
            let mut x:u8;
            let mut y:u8;
            loop{
                println!("insert value or press q to quit");
                value = std::io::stdin().read_line(&mut input).unwrap().to_string().parse().unwrap();
                if input == "q".to_string(){
                    break;
                }
                println!("insert value's x coordinate");
                x = std::io::stdin().read_line(&mut input).unwrap().to_string().parse().unwrap();
                println!("insert value's y coordinate");
                y = std::io::stdin().read_line(&mut input).unwrap().to_string().parse().unwrap();

                self.set_variable((x,y),value);
            }
        }

        pub fn print(&self){
            for i in 0..9{
                for j in 0..9{
                    if self.domains[i][j].len()==0{
                        print!("e ")
                    }
                    else if self.domains[i][j].len()==1{
                        print!("{} ",self.domains[i][j][0])
                    }
                    else{
                        print!("b ")
                    }
                    print!("\n")
                }
            }
        }
    }

}

mod backtracking{


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
                (new_variable.0/3==assigned_variable.0/3 && new_variable.1/3==assigned_variable.1/3){
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
                (new_variable.0/3==unassigned_variable.0/3 && new_variable.1/3==unassigned_variable.1/3){
                start_domain = p.get_domain(unassigned_variable);
                new_domain=consistency_helper(p,start_domain,new_variable);
                if new_domain.is_empty(){
                    inconsistency_found= true;
                }
                inferences.push((unassigned_variable,new_domain));
            }
        }
        return if !inconsistency_found {
            Ok(inferences)
        } else {
            Err("Failure".to_string())
        }

    }


    fn backtrack(p: &mut Sudoku) -> Result<&Sudoku,String>{
        if p.is_complete(){
            return Ok(p);
        }
        let var = select_unassigned_variable(&p);
        for value  in order_domain_values(p, var) {
            if is_consistent(p, var, value) {
                p.set_variable(var, value);
                let inferences = inference(&p, var);
                let go = match inferences{
                    Ok(..)=>true,
                    Err(..)=>false
                };
                if go {
                    p.add_inferences(inferences.unwrap() );
                    let result = backtrack(p);
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
        return Err("Failure".to_string());
    }
}





fn main() {

    println!("Hello, world!");
}
