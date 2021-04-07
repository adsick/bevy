use bevy::prelude::*;

fn main(){
    App::build().add_system(hello.system()).run();

}

fn hello(){
    println!("hello");

}