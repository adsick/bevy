mod keyboardal;
mod layout;

use keyboardal::*;

use layout::scdv;

use bevy::{input::{keyboard::KeyboardInput}, prelude::*, text::Text2dSize};
//use std::collections::HashMap;
//use bevy::utils::{HashMap, StableHashSet};

const REPEAT_INTERVAL: f32 = 0.2;
const DOWN_INTERVAL: f32 = 0.2;

#[derive(Bundle)]
struct Editor{
    mode: Mode,
    cursor: Cursor,
    #[bundle]
    text: Text2dBundle, //????
    scroll: Scroll,
    //commands vecdeque
    //text or line/s
    //cursor
}



struct Cursor {
    line: usize,
    col: usize,
    ind: Option<usize>, //where exactly put new chars
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            line: 0,
            col: 0,
            ind: None,
        }
    }
}

#[derive(Debug)]
enum Mode {
    //rename to editor mode
    //future replace with Bevy's states
    Normal,
    Insert,
    //Visual
}
#[derive(Debug)]
enum Command {
    PutChar(char),
    PutString(String),
    RemoveBefore,
    RemoveAfter,

    AddLine,
    RemoveLine,

    MoveCursorH(i16),
    MoveCursorV(i16),
    
    //MoveCursor(i16, i16)
    //Repeat(u16, Box<Command>)
}

struct UiCurrentMode; //bad structure
struct UiLines; //bad structure
struct Scroll {
    cur: f32,
    max: f32,
    acc: f32, //acceleration
              //dec: f32
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Cole".to_string(),
            ..Default::default()
        })
        .insert_resource(Mode::Normal)
        .insert_resource(KeyStrokes::new())
        .add_event::<Command>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(input.system().label("input"))
        .add_system(control.system().after("input").label("control"))
        .add_system(process.system().after("control"))
        .add_system(update_mode_label.system().after("input"))
        .add_system(scroll.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let text_style_body = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "This text is in the 2D scene.",
                text_style_body,
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            transform: Transform::from_xyz(400.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(UiLines)
        .insert(Scroll {
            cur: 0.0,
            max: 1000.0,
            acc: 1000.0,
        });

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Mode: ",
                TextStyle {
                    font,
                    font_size: 30.0,
                    color: Color::FUCHSIA,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(-400.0, -200.0, 0.0),
            ..Default::default()
        })
        .insert(UiCurrentMode);

    commands.insert_resource(Cursor::default());
}

fn input(
    mut key_evr: EventReader<KeyboardInput>,
    mut keystrokes: ResMut<KeyStrokes>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();

    keystrokes.update(dt); //comes first to allow 0.0 durations

    key_evr
        .iter()
        .for_each(|k| keystrokes.eat_keyboard_input(k));

}

fn control(
    mut com_evw: EventWriter<Command>,
    mut m: ResMut<Mode>,
    keystrokes: Res<KeyStrokes>,
    mut scroll: Query<&mut Scroll>,
    time: Res<Time>,
) {
    let p = keystrokes.get_just_pressed();
    if !p.is_empty(){
        println!("just pressed: {:?}", keystrokes.get_just_pressed());
    }

    let d = keystrokes.get_down();
    if !d.is_empty(){
        println!("down: {:?}", keystrokes.get_down());
    }

    let dp = keystrokes.get_double_pressed();
    if !dp.is_empty(){
        println!("double-pressed: {:?}", keystrokes.get_double_pressed());
    }

    let p = keystrokes.get_just_pressed().into_iter().next();
    let d = keystrokes.get_down().into_iter().next();
    let dp = keystrokes.get_double_pressed().into_iter().next();

    let dt = time.delta_seconds();

    let scroll = scroll.iter_mut();

    match *m {
        Mode::Normal => {
            if let Some(p) = p {
                match p {
                    34 => *m = Mode::Insert,
                    //35 => com_evw.send(Command::RemoveLine), //prob to be changed
                    36 => com_evw.send(Command::MoveCursorV(1)),
                     //J move cursor left (down in the future)
                    37 => com_evw.send(Command::MoveCursorH(1)),
                     //H move cursor right
                    _ => {}
                }
            }
            if let Some(d) = d { //a key is down
                match d {
                    50 => scroll.for_each(|mut s| s.cur = (s.cur - (s.acc * dt)).clamp(-s.max, 0.0)),
                    51 => scroll.for_each(|mut s| s.cur = (s.cur + (s.acc * dt)).clamp(0.0, s.max)),
                     //scrollup
                    

                    //57 => *m = Mode::Input,
                    _ => {}
                }
            }


            if let Some(dp) = dp {
                match dp {
                    35 => com_evw.send(Command::RemoveLine),
                    _ => {}
                }
            }
        }
        Mode::Insert => {
            if let Some(p) = p {
                if p == 1 || p == 58 {
                    //esc or caps key to go back to Normal mode
                    *m = Mode::Normal;
                } if p == 28 { //enter
                    com_evw.send(Command::AddLine);
                }
                else {
                    //let mut nt = scdv(sc).to_string();

                    com_evw.send(Command::PutChar(scdv(p)));
                }
            }
        }
    }
}

fn process(
    mut com_evr: EventReader<Command>,
    mut cursor: ResMut<Cursor>,
    mut txt: Query<&mut Text, With<UiLines>>,
    mut m: ResMut<Mode>,
) {

    //let text = &mut txt.iter_mut().next().unwrap().sections[0].value;

    for com in com_evr.iter() {
        println!("got a command! {:?}", com);

        match com {
            &Command::MoveCursorH(d) => { //put this code inside of the Cursor itself 
                if d < 0{
                    cursor.col-=d.wrapping_abs() as usize;
                } else {
                    cursor.col+=d as usize;
                }
                cursor.ind = None;
                println!("Moved cursor. col: {}", cursor.col);
            },

            &Command::MoveCursorV(d) =>{//put this code inside of the Cursor itself 
                if d < 0{
                    cursor.line-=d.wrapping_abs() as usize;
                } else {
                    cursor.line+=d as usize;
                }
                cursor.ind = None;
                println!("Moved cursor. line: {}", cursor.line);
            }

            Command::AddLine => {
                if let Some(ts) = txt.single_mut().unwrap().sections.get_mut(cursor.line){ //bug: does nothing when there are no lines
                    let style = ts.style.clone();
                    cursor.line+=1;
                    cursor.ind = None;
                    if ts.value.chars().last() != Some('\n'){ //kostyl
                        ts.value.push('\n');
                    }
                    txt.single_mut().unwrap().sections.insert(cursor.line, TextSection{value: "new text section!".to_string(), style});
                }



            }

            Command::RemoveLine => {
                println!("removing a line... cursor.line: {}", cursor.line);
                txt.iter_mut().next().unwrap().sections.remove(cursor.line);
                //text.clear();
                cursor.line = cursor.line.saturating_sub(1);
                cursor.col = 0;
                cursor.ind = None;
                println!("removed a line! cursor.line: {}", cursor.line);
            }
            &Command::PutChar(ch) => {
                //refactor to another function/system or validate(?) the ind on mutations directly
                //updates the ind so it safely fits chars, also updates cursor.col to fit the line

                if let Some(ts) = txt.single_mut().unwrap().sections.get_mut(cursor.line){
                    if cursor.ind.is_none() {
                        cursor.ind = Some(0);
                        let mut colmax = 0;
                        for (n, (i, c)) in ts.value.char_indices().enumerate() {
                            print!("({}; {})", c, n);
                            if n == cursor.col {
                                break;
                            }
                            colmax = n + 1;
                            cursor.ind = Some(i + 1);
                        }
                        cursor.col = colmax; //limits the col to the end of the line
                        println!();
                    }
    
                    ts.value.insert(cursor.ind.unwrap_or(0), ch);
                    if let Some(i) = &mut cursor.ind {
                        let l = ch.len_utf8();
                        *i += l; //updates the ind
                        if l > 0 {
                            //probably unnecessary check
                            cursor.col += 1; //char printed, update the col
                        }
                    }
                    println!("cursor: col {}, ind {:?}", cursor.col, cursor.ind);

                } else {
                    println!("cursor out of range! (cursor.line: {})", cursor.line);
                }

            }
            _ => {}
        }
    }
}

fn scroll(time: Res<Time>, mut q: Query<(&mut Transform, &mut Scroll)>) {
    let dt = time.delta_seconds();
    for (mut transform, mut scroll) in q.iter_mut() {
        //println!("scroll.cur: {}", scroll.cur);
        transform.translation.y += dt * scroll.cur;

        //scroll.cur-=scroll.cur.signum()*scroll.acc*dt; //rework this to completly new scroll system later 
    }
}

fn update_mode_label(mut t: Query<&mut Text, With<UiCurrentMode>>, m: Res<Mode>) {
    let mut t = t.single_mut().unwrap();

    match *m {
        Mode::Normal => t.sections[0].value = "NORMAL MODE".to_string(),
        Mode::Insert => t.sections[0].value = "INSERT MODE".to_string(),
    }
}
