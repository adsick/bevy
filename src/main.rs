use bevy::{core::FixedTimestep, prelude::*};

struct Animal;

struct Human;

struct Immortal;

struct Health {
    amount: u16,
}

struct Name {
    name: String,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<(Entity, Damage)>()
        .add_startup_system(setup.system())
        .add_system(
            emit_dmg
                .system()
                .with_run_criteria(FixedTimestep::step(1.0))
                .label("dmg"),
        )
        .add_system(
            recv_dmg
                .system()
                .with_run_criteria(FixedTimestep::step(1.0))
                .after("dmg"),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Human)
        .insert(Name {
            name: "Pashka".to_string(),
        })
        .insert(Health { amount: 100 })
        .insert(Immortal);
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

fn heal(mut q: Query<(&Name, &mut Health)>) {
    for (name, mut health) in q.iter_mut() {
        health.amount += 1;
        println!(
            "healed {} by 1 hp and now he has {} hp",
            name.name, health.amount
        );
    }
}

fn recv_dmg(
    mut q: Query<(Entity, &Name, &mut Health, Option<&Immortal>)>,
    mut er: EventReader<(Entity, Damage)>,
) {
    //let events: Vec<&(Entity, Damage)> = er.iter().collect();
    for (e, n, mut h, im) in q.iter_mut() {
        println!("name: {}", n.name);
        if im.is_some() {
            println!("he is immortal, so we go to the next one...");
            continue;
        }
        for ev in er.iter() {
            println!("ev: {:?}; ", ev);

            if ev.0 == e {
                h.amount -= ev.1.amount;
                println!("damaged entity: {:?}, now it has {}hp", e, h.amount);
            }
        }
        println!();
    }
    println!("\n");
}
#[derive(Debug)]
struct Damage {
    amount: u16,
}

fn emit_dmg(mut er: EventWriter<(Entity, Damage)>, q: Query<Entity>) {
    for e in q.iter() {
        println!("damage emitted: {:?}", e);
        er.send((e, Damage { amount: 4 }));
    }
}
