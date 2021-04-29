//DESIGNATIONS will be following:📔
//C!ircumvent- desire to do smth else to avoid ... creating temp value, etc.(et cetera)
#![feature(map_into_keys_values)]
extern crate env_logger;
extern crate log;

use std::num::{self};
//extern crate num-traits;
//extern crate retainer;
//extern crate cashed;
use std::sync::{Arc, Mutex};
//use std::thread;//use std::include_str;
use chrono::{prelude, DateTime, Duration, Local, Utc};//,DateTime, FixedOffset, Utc};
use std::fmt::{Debug, Display};//For boundary in struct's types
use std::collections::HashMap;
use env_logger::{Builder, Target};
use num_traits::{Num, NumCast};
use std::io::Write;
use log::{Record, Level, Metadata, SetLoggerError};
use log::{info, warn, LevelFilter};
use std::thread;
use std::fmt;
use std::path::Path;
use std::fs::File;
use std::error::Error as StdError;//**** 
type StdResult<T> = std::result::Result<T, Box<dyn StdError>>;//dyn for dynamic return  
//we don't know before execution what type will be returned (Also simply error without classification)
use std::marker::PhantomData;
//use retainer::cache::Cache; Only in async
//use cached::proc_macro::cached;

///For following purposes:(Controlling every needed parts of code)
///1. begin when the main func launch (*by default)
///2. after, update each time as needed to take in account next block of logic.
///Will add difference between one and previous in second member
///and log expired times for blocks
///3. Phantom data for holding live time of block
#[derive(Debug, Clone)]
pub(crate) struct GlobalExpiredTime<'ph, T: NumCast, V, P: ?Sized>
where P: 'ph + std::marker::Send,
V: Default + num_traits::ToPrimitive,
T: num_traits::Zero + num_traits::ToPrimitive + num_traits::NumCast {
//First-  time of entry in block (Local or UTC)
//Second- key: (for example simply) number of block (or associated name) 
//        value: time count (on whether it was done in threads or simply run the same block twise, thrice, fourfold etc.)
//(Option- maybe you don't want to take it into account, then counted time on this=0)
    LBlockT: Arc<Mutex<Vec
        <DateTime<Local>>>>, // u32- count times
    IntInfo: HashMap<T, (V, Option<u32>)>, //Int- intilligence (mark it)
    BlLife: PhantomData<&'ph P>}

impl<'ph, K, V, P> GlobalExpiredTime<'ph, K, V, P> where
//K must be the number! of block in program counted times.(as example)
//Default- zero, ToPrimitive + std::cmp::Ord- requirement NumCast::from, Send- to use in threads
K: num_traits::Zero + std::default::Default + num_traits::NumCast + num_traits::ToPrimitive +
Copy + Sync + std::cmp::Ord + std::hash::Hash + std::fmt::Debug,//, TimeSpec:
V: Default + Sync +std::fmt::Debug + Clone + Display + num_traits::NumCast + num_traits::ToPrimitive + std::cmp::Ord,
P: std::marker::Send {
    pub fn new(t_kind: Option<String>) -> Self {
        if let Some(kind_of_time) = t_kind {
            //.chars().flat_map(char::to_uppercase).collect::<String>();
            if kind_of_time.to_uppercase()=="UTC" {
                let now: DateTime<Utc> = Utc::now();
                println!("UTC now is: {}, so you had instantiated time in program TBC_eq(TransferBurguerCorrection_eq)", now.to_rfc2822());
                thread::sleep(std::time::Duration::from_millis(5000_u64));
            }
            else if kind_of_time.to_uppercase()=="LOCAL" {//How impl |"LOC"?
                let now: DateTime<Local> = Local::now();
                println!("Local time now is: {}, so you had instantiated time in program TBC_eq(TransferBurguerCorrection_eq)", now.to_rfc2822());
                thread::sleep(std::time::Duration::from_millis(5000_u64));
            }
        }
//Now only create fields and push them in structs
        let mut date_vec = Vec::<DateTime<Local>>::new();
        date_vec.push(Local::now());
        let LBlockT= Arc::new(Mutex::new(date_vec));
        let int_info= HashMap::new();
            Self{
                LBlockT, //equiv LBlockT: LBlockT
                IntInfo: int_info, //because we know only start point- no other info to push
                BlLife: PhantomData
            }
    }
//Print info about what have been finished: sec parameter set count of elements to output
    pub fn details(&mut self, detailed_output: Option<u32>, step_skip: Option<u32>){
        let lt = &Local::now();//C!ircumvent- don't know how to evade(workaround) creating new temp value 
        let lvec= self.LBlockT.lock().expect("Error with access to mutex");
        let vec_first= self.LBlockT.lock().expect("Error with access to mutex").first()
            .ok_or_else(|| lt);//Mb not good- temporary local here
        if !self.IntInfo.is_empty(){
                while let Some(mut detail_counter) = detailed_output{
                    println!("You choice detailed output with {} priority\r", detail_counter);
            //Check that amount of elements in vec > than you choose to output
                let vec_ge: bool= self.LBlockT.lock().unwrap().len() as u32 - detail_counter > 0_u32;
                let condition:bool = if vec_ge {!detail_counter== (0_u32 | u32::max_value())} else
                    {!self.LBlockT.lock().unwrap().is_empty() && !detail_counter== (0_u32 | u32::max_value())};
                        while condition{
                            let entry_t= &mut lvec.clone(); entry_t.reverse();
                            let elements = entry_t.iter();//Also
                            let mut blnum= self.IntInfo.keys();
                            let skip_step: u32;
                            if let Some(skiping) = step_skip{
                                if skiping < entry_t.len() as u32 {skip_step = skiping;}
                                else {skip_step= 1_u32;}
                            }
                            else {skip_step= 1_u32;}
                                for (k, el) in elements.enumerate()
                                    .filter(|&g| g.0 as u32 %skip_step==0_u32){
                                    let ell: &DateTime<chrono::Local>;
                                    let kzero= &K::default();
                                    if let Some(next_ell)= entry_t.get(k + 1){
                                        ell= next_ell;
                                    }else{ell= lt}
                                    let annotation = self.IntInfo.get(&NumCast::from(k).unwrap());
                                    println!("Time {element:>width1$} : №{number:>0width2$} - {hash_annotation:?},\n   Location of code block: {bl:?}",
                                    number= k,
                                    width2= 2,
                                    element= el,
                                    width1= 4,
                                    hash_annotation= annotation.unwrap(),
                                    bl= blnum.next().unwrap_or(kzero));
                                detail_counter-=1;
                                
                        //Also log in file what had been counted
                        let blnum= blnum.next().unwrap_or(kzero);
                        let blcount= self.IntInfo.get(blnum).unwrap();
                                log::debug!("Element {vecelem} by №{number:>0width2$} in vector\n
                                Duration: {dur:.4}\n
                                must be in {blnum:?} * {blcount}= {res}=?= {timevec}",
                                vecelem= el, number= k,  width2= 2, 
                                blnum= blnum, dur= blcount.0,//Save average time of the same evaluation block 
                                blcount= blcount.1.unwrap_or(0_u32),
                                res= NumCast::from(*blnum).unwrap_or(0_u32) * blcount.1.unwrap_or(0_u32),
                                timevec= el.signed_duration_since(*ell));}
                         }
                    }
                }
        else { 
            let elements = self.LBlockT.lock().expect("Error with access to mutex");
            for (k,el) in elements.iter().enumerate(){             
                println!("Time block in linear vec {}- {element:>}",
                k, element= el);
            }
        }
    }
    pub fn update_loc(&mut self, description_str: Option<&str>, output_time: Option<&Path>) -> StdResult<()>{
        use std::fs::OpenOptions;
        let mut description= String::new();//Will store in 
        let file_path;
        if let Some(desc) = description_str{
            description= String::from(desc);
        }
        let mut vec_locs= self.LBlockT.lock().expect("Error with access to mutex");
        let lt = Local::now();
        vec_locs.push(lt);
        let zero_k= &K::zero(); let sec_v= (V::default(), Some(0u32));//don't know why needed
        let key_num_last_block = self.IntInfo.iter().max_by_key(|entry | &entry.1.0).unwrap_or((&zero_k, &sec_v));
        //get value with max key block number
        //let mut counted_times= self.IntInfo.get(key_num_last_block.0).unwrap_or(&sec_v);
        let mut next_key_block: u32 = NumCast::from(*key_num_last_block.0).unwrap();
        next_key_block= next_key_block + 1u32;
        let next_key_: K= NumCast::from(next_key_block).unwrap();
        let last_dur: V = NumCast::from(
            (lt.signed_duration_since(*vec_locs.get(vec_locs.len() - 2).unwrap())) //Local
            .num_microseconds().unwrap()).unwrap();
        //Then increment counted times on that block
        let counter = self.IntInfo.entry(next_key_).or_insert((last_dur, Some(0_u32) ) );
        let mut cc: Option<u32>= Some(counter.1.unwrap_or(0u32));
        if let Some(mut c) = cc {c+= 1;cc= Some(c);}
        counter.1= cc;
        if let Some(fname) = output_time {
        file_path = fname;
        let mut file = OpenOptions::new().append(true).open(file_path).expect(
            "cannot open file");
         file.write_all(format!("New Block № {:?}: period {} times count {:?}\n", next_key_, counter.0, cc).as_bytes()).expect("write failed");
         file.write_all(description.as_bytes()).expect("write failed");
         println!("file append success");}
//Style::new().foreground(Blue).italic().paint(
        Ok(())
    }
    pub fn print_time(path: &Path) -> StdResult<()> {
        use std::io::{BufRead, BufReader};
    if path.extension().unwrap() == "txt" {
    let tf = File::open(path)?;
    let buffered = BufReader::new(tf);
    println!("{}", ansi_term::Colour::Green.dimmed().on(ansi_term::Colour::Blue).paint("This will be from file time:"));
    for line in buffered.lines() {
        println!("{}", line?);
    }}
    Ok(())
    }
    pub fn lbl(&self)-> StdResult<DateTime<Local>>{//last block local
        Ok(*self.LBlockT.lock().expect("Error with access to mutex")//.map_err(|e| e.into())
            .last().expect("Last doesn't exist"))
    }

    pub fn difnow(&self)-> StdResult<chrono::Duration>{
        let vec_locs= self.LBlockT.lock().expect("Error with access to mutex");
        let lt = Local::now();
        Ok(lt.signed_duration_since(*vec_locs
            .get(vec_locs.len() - 1).unwrap()))
    }

}

/*
    fn update_next_block(&'static mut self, print_flog: Option<(bool, Option<(bool, bool)>)>/* logger: Log*/ ){
//Check if there is something counted already earlier otherwise initialize
        //let mut gvec = &self.0.lock()
            //.unwrap_or_else(|_| GlobalExpiredTime::new(None).0.lock().expect("Internal break in mutex!"));
//Check the penultimate one
        let old_key_num= self.1.keys().max_by_key(|&key| key).unwrap();
    /* I want! .unwrap_or_else(| | match K::type{
                i32 => &Some(K.is_zero()),
                i64 => &Some(1),
                bool => std::process::exit(1),
                _ => println!("Type don't match requirements"),
            });*///: Vec<&K> 
        let last_num_block = self.1.get(&old_key_num); 
            //.collect().max().iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        const N: usize = 4;
        if let Some((pprint, llog)) = print_flog {
        //let mut threads = Vec::<chrono::Duration>::with_capacity(N);
           if pprint{
//will print last N  
                (0..N).for_each(|seq_num| { // <---- Closure 1
                //access last N in vec
                let arc_clone = Arc::clone(&self.0);
                let size_vec= arc_clone.lock().unwrap().len();
                    thread::spawn(move || {  // <---- Closure 2
                        let loc_clone = arc_clone.lock().unwrap();
                        let cur_loc= loc_clone.get(size_vec- seq_num as usize).unwrap();
                        let lt = &Local::now();
                        let next_loc= loc_clone.get(size_vec- seq_num +1 as usize).unwrap_or_else(|| lt);
                        let utc_time = DateTime::<Utc>::from_utc(next_loc.naive_utc(), Utc);
                        let dif_locs = next_loc.signed_duration_since(*cur_loc);
                        println!("Element {} was executed in {:?}\n",  size_vec - seq_num, dif_locs);
                        //threads.push(dif_locs);
                   });
               });
            }
            if let Some(should_log)= llog{
                let mut log_file = std::fs::File::create("log_time.txt").expect("create failed");
                env_logger::init();
                Builder::new()
                    .target(Target::Stdout)
                    .init();
                log::set_max_level(LevelFilter::Error);
                let arc_clone = Arc::clone(&self.0);
                let mut block_info = (self.1.keys(), self.1.values());
                let size_vec= arc_clone.lock().unwrap().len();
                let mut file = std::fs::OpenOptions::new().append(true).open("log_time.txt").expect(
                    "cannot open file");
                thread::spawn(move || {
                    use std::str;
                    let mut cur: usize= 0;
                    while let Some(vec_elem)= arc_clone.lock().unwrap().get(size_vec-cur){
                        println!("Block number {0:?} executed in {1:?}", block_info.0.next().unwrap().unwrap(),//excessive, i know)
                            block_info.1.next().unwrap());
                            //file.write_all(format!("Block number {:?} executed in {:?}\n".as_bytes(), String::from_str(block_info.0.next().unwrap().unwrap()).as_bytes(),
                            //block_info.1.next().unwrap().as_bytes()));
                            log::info!("Written {} time", cur);
                            cur+= 1;
                    };
                });
            }
        }//end log
    }
}
impl<'ph, K, V, P> log::Log for  GlobalExpiredTime<'ph, K, V, P> where
//K must be the number! of block in program counted times, 
K: Sync + Send + NumCast + std::cmp::Ord + std::hash::Hash + std::fmt::Debug,//, TimeSpec:
V: Sync + Send + std::fmt::Debug + Clone,
P: std::marker::Send {
    fn enabled(&self, metadata: &Metadata) -> bool {
       metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) -> StdResult<bool> {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("log_time/time_out.log")?;

        let config = Config::builder()
                .appender(Appender::builder().build("log_time", Box::new(logfile)))
                .build(Root::builder()
                .appender("log_time")
                .build(LevelFilter::Info))?;

                log4rs::init_config(config)?;
            Ok(true)
        }
    }

    fn flush(&self) {}
}      
}
    let b=
    if let time= OldMeasureTime{
        let gr_zero: bool=  ContinueMeasureTime.sub(time) > chrono::Duration::zero();
        gr_zero
    }
    else{false};
    if b{
        let ExpiredTime= ContinueMeasureTime- time;}
    else {ContinueMeasureTime= self.0}
    self.0 + time}*/
trait SplitTimeOnUnits{
    //fn 
}
fn main()-> Result<(), log::SetLoggerError>{
Ok(())    //let n= test_timing();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timing(){
        let builder = Builder::new()
            .is_test(true).filter_level(LevelFilter::Info);//.format_timestamp(Some(chrono::offset::Local::now()));
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
        
        //let mut  time: super::GlobalExpiredTime<u32, i32, i32>= GlobalExpiredTime::new(Some("UTC".to_owned()));
        const BLOCKS: usize= 10;
        for i in 0.. BLOCKS{
            //time.update_new();
        }
        //time.details(Some(1 as u8));
        //let accuracy: i64= 1_i64;
        //assert!(time.loc_block().signed_duration_since(Local::now()) < 
        //    Duration::microseconds(accuracy.pow(2)));
    }
}
