use bevy::{input::keyboard::KeyboardInput, prelude::*};
use std::collections::HashMap;
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
    RemoveLine,

    MoveCursorH(i16),
    MoveCursorV(i16),
    //MoveCursor(i16, i16)
    //Repeat(u16, Box<Command>)
}

struct UiCurrentMode;
struct UiLines;
struct Scroll {
    //used for scrolling, but actually is a good name for Vec<Line> thing
    cur: f32,
    max: f32,
    acc: f32, //acceleration
              //dec: f32
}

struct KeyPresses(HashMap<u32, KeyState>);

impl KeyPresses {
    fn new() -> Self {
        KeyPresses(HashMap::new())
    }
}
// struct KeyPresses{
//     keymap: HashMap<u32, KeyState>
// }
#[derive(Debug, PartialEq, Clone, Copy)]
enum KeyState {
    JustPressed,
    JustReleased,
    Pressed(f32), //seconds
    Released(f32),
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Cole".to_string(),
            ..Default::default()
        })
        .insert_resource(Mode::Normal)
        .insert_resource(KeyPresses::new())
        .add_event::<Command>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(input.system().label("input"))
        .add_system(process.system().after("input"))
        .add_system(update_mode_label.system().after("input"))
        .add_system(scroll.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let style_body = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "This text is in the 2D scene.",
                style_body,
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Right,
                },
            ),
            transform: Transform::from_xyz(-400.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(UiLines)
        .insert(Scroll {
            cur: 0.0,
            max: 100.0,
            acc: 10.0,
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
                    horizontal: HorizontalAlign::Right,
                },
            ),
            transform: Transform::from_xyz(-600.0, 000.0, 0.0),
            ..Default::default()
        })
        .insert(UiCurrentMode);

    commands.insert_resource(Cursor::default());
}

fn input(
    mut query: Query<(&mut Transform, &mut Text, &mut Scroll), With<UiLines>>,
    mut key_evr: EventReader<KeyboardInput>,
    mut com_evw: EventWriter<Command>,
    mut cursor: ResMut<Cursor>,
    mut time: Res<Time>,
    mut kprs: ResMut<KeyPresses>,
    mut m: ResMut<Mode>,
) {
    let dt = time.delta_seconds();

    //updates continiously&independently of key events.
    for ks in kprs.0.values_mut() {
        if let KeyState::Released(t) = ks {
            *ks = KeyState::Released(*t + dt);
        } else if let KeyState::Pressed(t) = ks {
            *ks = KeyState::Pressed(*t + dt);
        } else if let KeyState::JustReleased = ks {
            *ks = KeyState::Released(0.0);
        } else if let KeyState::JustPressed = ks {
            *ks = KeyState::Pressed(0.0);
        }
    }

    println!("all the keypresses: {:?}", kprs.0);

    for (mut transform, mut text, mut scroll) in query.iter_mut() {
        for kev in key_evr.iter() {
            //println!("{:?}", kev);

            let sc = kev.scan_code;
            let kc = kev.key_code;
            let st: KeyState;

            if kev.state == bevy::input::ElementState::Pressed {
                if let Some(ks) = kprs.0.get_mut(&sc) {
                    if let KeyState::Released(_) = ks {
                        *ks = KeyState::JustPressed;
                    } 
                    st = *ks;
                } else {
                    st = KeyState::JustPressed;
                    kprs.0.insert(sc, st);
                }
            } else {
                //key released
                st = KeyState::JustReleased;
                let ks = kprs.0.get_mut(&sc).unwrap(); //Released key must be present
                *ks = st;
            }

            //println!("sc: {} st: {:?}", sc, st);

            match *m {
                Mode::Normal => match sc {
                    34 => *m = Mode::Insert,
                    35 => com_evw.send(Command::RemoveLine), //prob to be changed
                    36 => {
                        cursor.line += 1;
                        cursor.ind = None;
                    } //J move cursor down,
                    37 => {
                        cursor.col += 1;
                        cursor.ind = None;
                    } //H move cursor right, incremental ind optimization may be an option, now stick to lazy ind updating

                    50 => {
                        scroll.cur -= scroll.max; //send event here?
                    } //scrolldown //-= (scroll.max - scroll.cur)*scroll.acc*dt;
                    51 => {
                        scroll.cur += scroll.max;
                    } //scrollup
                    _ => scroll.cur = 0.0, //wrong, there is no code in fact

                    //57 => *m = Mode::Input,
                    _ => {}
                },
                Mode::Insert => {
                    if sc == 1 || sc == 58 {
                        //esc or caps key to go back to Normal mode
                        *m = Mode::Normal;
                    } else if kc == Some(KeyCode::Back) {
                        com_evw.send(Command::RemoveLine); //prob to be changed
                    } else {
                        //let mut nt = scdv(sc).to_string();

                        com_evw.send(Command::PutChar(scdv(sc)));
                    }
                }
            }
        }
    }
}

fn scroll(time: Res<Time>, mut q: Query<(&mut Transform, &mut Scroll)>) {
    let dt = time.delta_seconds();
    for (mut transform, scroll) in q.iter_mut() {
        transform.translation.y += dt * scroll.cur;
    }
}

fn process(
    mut com_evr: EventReader<Command>,
    mut cursor: ResMut<Cursor>,
    mut txt: Query<&mut Text>,
    mut m: ResMut<Mode>,
) {
    let text = &mut txt.iter_mut().next().unwrap().sections[0].value;

    for com in com_evr.iter() {
        println!("got a command! {:?}", com);

        let com = com;

        match com {
            Command::RemoveLine => {
                text.clear();
                cursor.col = 0;
                cursor.ind = None;
            }
            //updates the ind so it safely fits chars, also updates cursor.col to fit the line
            &Command::PutChar(ch) => {
                //refactor to another function/system or validate(?) the ind on mutations directly
                if cursor.ind.is_none() {
                    cursor.ind = Some(0);
                    let mut colmax = 0;
                    for (n, (i, c)) in text.char_indices().enumerate() {
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

                text.insert(cursor.ind.unwrap_or(0), ch);
                if let Some(i) = &mut cursor.ind {
                    let l = ch.len_utf8();
                    *i += l; //updates the ind
                    if l > 0 {
                        //probably unnecessary check
                        cursor.col += 1; //char printed, update the col
                    }
                }
                println!("cursor: col {}, ind {:?}", cursor.col, cursor.ind);
            }
            _ => {}
        }
    }
}

fn update_mode_label(mut t: Query<&mut Text, With<UiCurrentMode>>, m: Res<Mode>) {
    let mut t = t.single_mut().unwrap();

    match *m {
        Mode::Normal => t.sections[0].value = "NORMAL MODE".to_string(),
        Mode::Insert => t.sections[0].value = "INSERT MODE".to_string(),
    }
}

fn scdv(sc: u32) -> char {
    //scancode to dvorak
    match sc {
        2 => '1',
        3 => '2',
        4 => '3',
        5 => '4',
        6 => '5',
        7 => '6',
        8 => '7',
        9 => '8',
        10 => '9',
        11 => '0',
        16 => '\'',
        17 => ',',
        18 => '.',
        19 => 'p',
        20 => 'y',
        21 => 'f',
        22 => 'g',
        23 => 'c',
        24 => 'r',
        25 => 'l',
        26 => '/',
        27 => '=',
        28 => '\n', //will create a new section in the future
        30 => 'a',
        31 => 'o',
        32 => 'e',
        33 => 'u',
        34 => 'i',
        35 => 'd',
        36 => 'h',
        37 => 't',
        38 => 'n',
        39 => 's',
        40 => '-',
        43 => '\\',
        44 => ';',
        45 => 'q',
        46 => 'j',
        47 => 'k',
        48 => 'x',
        49 => 'b',
        50 => 'm',
        51 => 'w',
        52 => 'v',
        53 => 'z',
        57 => ' ',
        _ => '\0',
    }
}
