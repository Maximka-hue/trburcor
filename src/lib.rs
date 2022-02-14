//#[crate_type = "staticlib"]
//#![allow(unused)]
//Deduce type of variable
//library to store argument parsing and file structures from which will read data
//#![feature(core_intrinsics)]//To infer explicit type
//#![feature(option_expect_none)] //for method finding duplicates
//#![feature(format_args_capture)]
/* Building struct */
#[macro_use]
pub use derive_builder::Builder;
pub use std::borrow::Cow;
//use once_cell::sync::OnceCell;
use ansi_term::{self};
//use std::cell::{RefCell, Cell, RefMut};
/* Regex */
extern crate lazy_static;
pub use lazy_static::lazy_static;
pub use regex::Regex;
/* (Se/De)serialization - Nothing now*/

pub(crate) use std::thread;
pub use std::convert::Into;

/* Stylisation */
pub(crate) use tutil::crayon::Style;
pub(crate) use tutil::crayon::Color::{Red, Blue};
pub(crate) use std::process::Command;
pub(crate) use std::sync::Mutex;
/* Timing */
pub(crate) use std::time::{self, Instant};
pub(crate) use std::io::{self, prelude, stdin, Read, BufRead, BufReader, ErrorKind};//Создать записать в файл
/* Working with Files */
//use itertools::Itertools;
pub use std::path::{self, PathBuf, Path};
pub use structopt::StructOpt;
pub(crate) use std::{format};

const LUCKY_QUANTITY_KNOWLEDGE: u8 = 5; // also need from keyboard
#[macro_use]
#[path="./lib/src/time_code.rs"]
pub(crate) mod time_code;
//pub: So i will use(reexport) it also in main
use time_code::GlobalExpiredTime;
use ansi_term::Colour::*;


//____________________________Input file data_____________________________________________________
//This in case to create new file_process structure manually
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate_parameters"))]
pub struct FileParametres{
    #[builder(public)]
    pub eq_type:i8,
    #[builder(default = "(0 as f32, 1 as f32)")]
    pub margin_domain:(f32,f32),
    pub time_eval_period_stage: (f32,f32),
    pub bound_type:i8,
    pub init_type:i8,
    pub init_conditions: (f32,f32, Option<f32>, Option<f32>),
    pub quantity_split_nodes: u32,
    #[builder(setter(into))]
    pub n_corant:f32,
//#[builder(setter(into, strip_option), default)]- don't work
    pub add_args: (Option<TypeTsk>, Option<i8>, Option<bool>)//will be last background_mc additional_correction
    //pub add_args: Vec<Option<TypeTsk>, Option<i8>, Option<bool>> I want like this, but don't know way
}
//Specificity of the entered argument (type of equation)
//Специфика введенного первого аргумента (типа уравнения)
#[derive(Debug, Clone, StructOpt, PartialEq)]
pub enum TypeTsk{
    BURGER {b: String},//It's only to choose type of equation 
    TRANSFER {a: f32},  //,u0_1:f32,u0_2:f32,u0_3:Option<f32>},
}

//___________________________________________________________________________________________________
//Little check for parameters
impl FileParametresBuilder {
    fn validate_parameters(&self) -> std::result::Result<(), String>{//io::ErrorKind
        if let Some(ref eq_type) = self.eq_type {
            match *eq_type {
                i if i < 0 => {pt("First less than 0, no such type equation",None); println!("Nothing right in equation!");panic!("Invalid number: {}", i)},//ErrorKind::InvalidData
                i if i > 1 => {pt("First more than one, no such type equation",None); println!("Nothing right in equation!");panic!("Invalid number: {}", i)},
                _ => Ok(())
            }
        } 
        else if self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).0 < self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).1 {
            println!("Incorrect time specification: {}", self.time_eval_period_stage.unwrap().0);
            pt("Please correct programm time boundary", None);
            println!("Nothing right in time!");panic!("Invalid time: must be {:.3}>{:.3}", self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).0,
            self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).1)}
        
        else if (self.margin_domain.unwrap_or((0_f32, 0_f32)).0 - self.margin_domain.unwrap_or((0_f32,0_f32)).1).abs()== std::f32::MIN 
        {
            println!("Incorrect Domain input");
            panic!("Domain is 0!");
        }//Check not to divide further by 0 in Transfer task
        else if self.eq_type.unwrap_or(0_i8) == 0
            {if let Some(velocity_) = self.add_args.clone().unwrap().0
                {// if there is smth in additional arguments...
                if velocity_ == (TypeTsk::TRANSFER{a: 0_f32}) {
                println!("Transfer build must be not 0!");
                pt("Please correct transfer parameter or change type equation", None);
                println!("Nothing right in time!");panic!("{:?}",TypeTsk::TRANSFER{a:0_f32})}
                else{println!("Input transfer velocity is {:?}", velocity_);
                    return Ok(())
                    }
                }Ok(())}
        else {
            thread::sleep(time::Duration::from_millis(500_u64));
            Ok(())
            }
    }
}
//Then will be blocks that only for me to understand rust!(Maybe you will do another initializations,....)
//Further mark it like ***************************************************************************
//Little shorthand for debug
pub fn pt<'a, S: AsRef<str>>(data: S, text: Option<S>)
where S: Into<Cow<'a, str>>{
    println!("{}, {:>?}", data.into(), text.as_ref().map(|r| r.as_ref().to_string()));
}
#[macro_export]
#[warn(unused_macros)]
macro_rules! pt {
    ($a: expr) => {
        pt($a, None)
    };
    ($a: expr, $b: expr) =>{
        pt($a , $b)
    };
}
pub fn ph(data1: &Vec<String>){//print_smth
        for (i,s) in data1.iter().enumerate(){
            print!("{} \t", s);
            if i%2 ==0 {print!("\n");}}}
#[macro_export]
#[warn(unused_macros)]
macro_rules! ph {
    ($a: expr) => {
        ph($a)
    }
} 

impl FileParametres {
    pub fn first_initializing() -> std::result::Result<FileParametres, ErrInTransferTask>{
        let datas = FileParametresBuilder::default()
            .eq_type(0)
            .time_eval_period_stage((0 as f32, 0 as f32))
            .margin_domain((0 as f32, 0 as f32))
            .bound_type(0)
            .init_type(0)
            .init_conditions((0_f32, 0_f32, None, None))
            .quantity_split_nodes(0)
            .n_corant(0 as f32)
            .add_args((Some(TypeTsk::BURGER{b:"Some(None) speed initial".to_string()}),Some(0_i8),Some(false)))
            .build().unwrap();//.map_err(|_| ErrInTransferTask::FileParams)
        println!("{}", ansi_term::Colour::Green.paint("Initializing struct with default zeros\n"));
        Ok(datas)}     
//*****************************************************************************************************************************  
    pub fn new(eq_type:String,
        margin_domain:(f32,f32),
        time_eval_period_stage:(f32,f32),
        bound_type: String,
        init_type: String,
        init_conditions: (f32,f32,f32,f32),
        quantity_split_nodes: String,//Option<i32>,
        n_corant: String,
        add_args: (TypeTsk, i8, bool)) -> FileParametres {
            FileParametres{eq_type: eq_type.trim().parse::<i8>().unwrap(), //ret: trim-slice, parse- to specified type
                margin_domain: (margin_domain.0, margin_domain.1),
                bound_type: bound_type.trim().parse().expect(" "),
                init_type: init_type.trim().parse().unwrap(),
                init_conditions:(init_conditions.0, init_conditions.1, Some(init_conditions.2), Some(init_conditions.3)),
                quantity_split_nodes : quantity_split_nodes.trim().parse().unwrap(),
                n_corant : n_corant.trim().parse().unwrap(),
                time_eval_period_stage, 
                add_args: (Some(add_args.0), Some(add_args.1), Some(add_args.2)),
        }
    }
//*****************************************************************************************************************************
}//Some(SelectSpecTypePrp::None),/**/
#[derive(Debug, Clone)]
pub enum ErrInTransferTask{
    DebOptim(i32),
    FileParams(i32),
    ExecMain(String)
}

use std::io::Write;
fn main()-> Result<(), ErrInTransferTask>{
let mut search_error: ErrInTransferTask;//will find error in separate part of program
let mut output_path = PathBuf::new();//Will write results into


    Ok(())
}
fn count1_norm(prev: Vec<f32>,current: Vec<f32>, temp: Vec<f32>){
    
}
fn goodbye() -> String {
    "さようなら".to_string()
}
fn test<T: AsRef<str>>(inp: &[T]) {
    for x in inp { print!("{} ", x.as_ref()) }
    println!("");
}
//Let's create struct to save vector with initial values and maximal velocity in it.
use std::fmt::{Debug, Display};
#[derive(Builder, Debug, PartialEq)]
struct IVdata<T: Display>{
    ivector: Vec<T>,
#[builder(default = "100")]
    ivecs: usize 
}
impl<T: Display> IVdata<T> 
{/*
    fn rung<U: Sized, V: Debug> (c:U, w:U, h: U, vec_prev: V, dx: U, debug_q: Option<i16>, domain: Option<(i16, i16)>)
    where U: Debug
    {
        println!("Ступенька под уравнеие переноса)"); 
        //We can't access values in vector by f32, so we count steps from left bound
                        let dip = (w / dx) as u32; //Количество кусочков внутри, dx=0.01,0.1...;
                        let start = ((c-w/ 2 as U)/dx); // as f32 | Two times unwrap the same!
                        if start >  domain.unwrap_or((0_i16,0_i16)).0 as f32 || start< domain.unwrap_or((0_i16,0_i16)).0  {
                            println!("Левая|правая точка ступенька вне заданной области");
                            panic!("Out of domain!");} 
                        for n in 0..dip{
                            vec_prev[start as usize + n as usize] = h.max(-h) as f32;
                            if n%25==0 {println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start , vec_prev[n as usize+start as usize]);}}}
    }*/
}
//For simplicity without default)
#[warn(unused_macros)]
macro_rules! rung {
    ($a: expr, $b: expr, $c: expr, $d: expr,$step: expr, $e: expr) => {
        rung($a, $b, $c, $d, $step, $e)
    };
    ($a: expr, $b: expr, $c: expr, $d: expr, $step: expr) => {
        rung($a, $b, $c, $d, $step, step*100)
    };
}

/*
for entry in WalkDir::new("foo").min_depth(1).max_depth(3) {
        println!("{}", entry?.path().display());
    }
    */
/*
#[cfg(test)]
mod tests {
use super::*;
fn try_parse(){
    let opt = DebOpt::from_args();
    println!("{:?}", opt);
    }
fn main(){
    try_parse()
    }
}*/
