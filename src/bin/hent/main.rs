mod editor;
mod keyboardal;
mod layout;


use editor::{Cursor, Editor, EditorCommand, EditorCommands, EditorMode, Scroll, ScrollState};
use keyboardal::*;

use layout::Layout;

use bevy::{input::keyboard::KeyboardInput, prelude::*};

const REPEAT_INTERVAL: f32 = 0.2;
const DOWN_INTERVAL: f32 = 0.2;

struct UiCurrentMode; //bad structure



fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Hentype".to_string(),
            ..Default::default()
        })
        .insert_resource(KeyStrokes::new())
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

    //commands.insert_resource(Cursor::default());

    let editor = Editor {
        mode: EditorMode::Normal,
        cursor: Cursor::default(),
        text: Text2dBundle {
            text: Text::with_section(
                "Meet the hent editor!",
                text_style_body,
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            transform: Transform::from_xyz(400.0, 0.0, 0.0),
            ..Default::default()
        },
        scroll: Scroll {
            cur: 0.0,
            max: 2000.0,
            acc: 4000.0,
            state: ScrollState::Idle,
        },
        commands: EditorCommands::new(),
        layout: Layout::Dvorak,
    };

    commands.spawn_bundle(editor);
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
    //mut com_evw: EventWriter<Command>,
    //mut m: ResMut<Mode>,
    keystrokes: Res<KeyStrokes>,
    mut editor_query: Query<(
        &mut EditorMode,
        &mut EditorCommands,
        &mut Layout,
        &mut Scroll,
    )>,
) {
    //takes one of each
    let p = keystrokes.get_pressed().into_iter().next();
    let jp = keystrokes.get_just_pressed().into_iter().next();
    let d = keystrokes.get_down().into_iter().next();
    let dp = keystrokes.get_double_pressed().into_iter().next();

    let (mut m, mut c, mut l, mut s) = editor_query.single_mut().unwrap();

    match *m {
        EditorMode::Normal => {
            if let Some(jp) = jp {
                match jp {
                    34 => *m = EditorMode::Insert,
                    //35 => com_evw.send(Command::RemoveLine), //prob to be changed
                    36 => {
                        if keystrokes.get_pressed().contains(&57) {
                            c.push(EditorCommand::MoveCursorUp(1))
                        } else {
                            c.push(EditorCommand::MoveCursorDown(1))
                        }
                    }

                    //J move cursor left (down in the future)
                    37 => {
                        if keystrokes.get_pressed().contains(&57) {
                            c.push(EditorCommand::MoveCursorLeft(1))
                        } else {
                            c.push(EditorCommand::MoveCursorRight(1))
                        }
                    }
                    //H move cursor right
                    _ => {}
                }
            }
            if let Some(p) = p {
                //a key is down
                match p {
                    50 => s.state = ScrollState::ScrollingDown,
                    51 => s.state = ScrollState::ScrollingUp,
                    //scrollup

                    //57 => *m = Mode::Input,
                    _ => {}
                }
            } else {
                 
            }

            if let Some(dp) = dp {
                match dp {
                    35 => c.push(EditorCommand::RemoveLine),
                    _ => {}
                }
            }
        }
        EditorMode::Insert => {
            if let Some(p) = jp {
                if p == 1 || p == 58 {
                    //esc or caps key to go back to Normal mode
                    *m = EditorMode::Normal;
                } else if p == 42 {
                    l.switch();
                } else if p == 28 {
                    //enter
                    c.push(EditorCommand::AddLine);
                } else {
                    //let mut nt = scdv(sc).to_string();

                    c.push(EditorCommand::PutChar(l.scch(p)));
                }
            }
        }
    }
}

fn process(
    //mut com_evr: EventReader<Command>,
    mut query: Query<(&mut Text, &mut EditorCommands, &mut Cursor)>,
) {
    let (mut txt, mut commands, mut cursor) = query.single_mut().unwrap();

    while commands.busy() {
        let c = commands.pop().unwrap();

        println!("got a command! {:?}", c);

        match c {
            EditorCommand::MoveCursorDown(p) => {
                cursor.move_down(p);
                println!("Moved cursor. line: {}", cursor.line);
            }
            EditorCommand::MoveCursorUp(p) => {
                cursor.move_up(p);
                println!("Moved cursor. line: {}", cursor.line);
            }

            EditorCommand::MoveCursorRight(p) => {
                cursor.move_right(p);
                println!("Moved cursor. col: {}", cursor.col);
            }

            EditorCommand::MoveCursorLeft(p) => {
                cursor.move_left(p);
                println!("Moved cursor. col: {}", cursor.col);
            }

            EditorCommand::AddLine => {
                if let Some(ts) = txt.sections.get_mut(cursor.line) {
                    //bug: does nothing when there are no lines
                    let style = ts.style.clone();
                    cursor.line += 1;
                    cursor.ind = None;
                    if ts.value.chars().last() != Some('\n') {
                        //kostyl
                        ts.value.push('\n');
                    }
                    txt.sections.insert(
                        cursor.line,
                        TextSection {
                            value: "new text section!".to_string(),
                            style,
                        },
                    );
                }
            }

            EditorCommand::RemoveLine => {
                println!("removing a line... cursor.line: {}", cursor.line);
                if txt.sections.get(cursor.line).is_some() {
                    txt.sections.remove(cursor.line);
                    //text.clear();
                    cursor.line = cursor.line.saturating_sub(1);
                    cursor.ind = None;
                    println!("removed a line! cursor.line: {}", cursor.line);
                } else {
                    println!("this line does not exists ({})", cursor.line);
                }
            }
            EditorCommand::PutChar(ch) => {
                //refactor to another function/system or validate(?) the ind on mutations directly
                //updates the ind so it safely fits chars, also updates cursor.col to fit the line

                if let Some(ts) = txt.sections.get_mut(cursor.line) {
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
                }
            }
            _ => {}
        }
    }
}

fn scroll(time: Res<Time>, mut q: Query<(&mut Transform, &mut Scroll)>) {
    let dt = time.delta_seconds();
    for (mut transform, mut s) in q.iter_mut() {
        //println!("scroll.cur: {}", s.cur);
        transform.translation.y += dt * s.cur;
        
        match s.state {
            ScrollState::ScrollingUp => s.cur = (s.cur + (s.acc * dt)).clamp(0.0, s.max),
            ScrollState::ScrollingDown => s.cur = (s.cur - (s.acc * dt)).clamp(-s.max, 0.0),
            ScrollState::Idle => {
                if s.cur.abs() < 10.0{
                    s.cur = 0.0;
                }else{
                    s.cur -= s.cur.signum()*s.acc*dt;
                }
            }
        }
        s.state = ScrollState::Idle;
    }
}

fn update_mode_label(
    mut t: Query<&mut Text, With<UiCurrentMode>>,
    status_query: Query<(&EditorMode, &Cursor)>,
) {
    let mut t = t.single_mut().unwrap();

    if let Some((m, c)) = status_query.single().ok() {
        match m {
            EditorMode::Normal => {
                t.sections[0].value = format!("NORMAL MODE\nline: {} col: {}", c.line, c.col)
            }
            EditorMode::Insert => {
                t.sections[0].value = format!("INSERT MODE\nline: {} col: {}", c.line, c.col)
            }
        }
    }
}
