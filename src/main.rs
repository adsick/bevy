use bevy::{core::FixedTimestep, prelude::*};

struct Animal;

struct Human;

struct Health {
    amount: u16,
}

struct Name {
    name: String,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(hello.system().with_run_criteria(FixedTimestep::step(1.0)))
        .add_system(heal.system().with_run_criteria(FixedTimestep::step(0.5)))
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Human)
        .insert(Name {
            name: "Pashka".to_string(),
        })
        .insert(Health { amount: 100 });
    commands
        .spawn()
        .insert(Human)
        .insert(Name {
            name: "Oleg".to_string(),
        })
        .insert(Health { amount: 200 });
    commands
        .spawn()
        .insert(Animal)
        .insert(Name {
            name: "Oscar".to_string(),
        })
        .insert(Health { amount: 50 });
}

fn hello(time: Res<Time>, mut query: Query<(&Name, &mut Health)>) {
    for (n, mut h) in query.iter_mut() {
        h.amount -= 5;
        println!(
            "hello, {}. you have been damaged and now you have {} hp",
            n.name, h.amount
        );
    }
    println!("time is {:?}", time.seconds_since_startup());
}

fn heal(mut q: Query<(&Name, &mut Health), (Without<Human>)>) {
    for (name, mut health) in q.iter_mut(){
        health.amount += 1;
        println!(
            "healed {} by 1 hp and now he has {} hp",
            name.name, health.amount
        );
    }
}
