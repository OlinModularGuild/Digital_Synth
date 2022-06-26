use dasp::{signal, Sample, Signal};
use std::sync::mpsc;
use std::any::type_name;
use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::LinkedList;

struct wave{
    freq: f32,
    op: i16, // 1 - add, 2 - mult, 3 - figure out from there
    form: i16, // 1 - sine, 2 - square, 3 - saw
}

pub fn gen_wave<T>(list: &mut LinkedList<Wave>) 
where T: cpal::Sample, 
{
    :wave
}

pub fn del_wave(){
    println!("deleted wave")
}