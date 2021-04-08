use bevy::{input::keyboard::KeyboardInput, prelude::*};

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

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            title: "Cole".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2d camera
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "This text is in the 2D scene.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 60.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Right,
            },
        ),
        transform: Transform::from_xyz(-400.0, 0.0, 0.0),
        ..Default::default()
    });

    commands.insert_resource(Cursor::default());
}

//TODO rename
fn animate(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Text)>,
    mut key_ev: EventReader<KeyboardInput>,
    mut cursor: ResMut<Cursor>,
) {
    // `Transform.translation` will determine the location of the text.
    // `Transform.scale` and `Transform.rotation` do not yet affect text (though you can set the
    // size of the text via `Text.style.font_size`)
    for (mut transform, mut text) in query.iter_mut() {
        //transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;

        for kev in key_ev.iter() {
            println!("{:?}", kev);
            if kev.state == bevy::input::ElementState::Pressed {
                if kev.key_code == Some(KeyCode::Back) {
                    //fixes needed
                    if cursor.col > 0 {
                        if let Some(ref mut ind) = cursor.ind {
                            // if let Some(c) = text.sections[0].value.get(*ind){
                            //     obtain len of c to go back and then remove the right char...
                            // }
                            let c = text.sections[0].value.remove(*ind);

                            *ind = ind.saturating_sub(c.len_utf8());

                            let ind = *ind;

                            cursor.col -= 1;

                            println!("Back ind: {}, col: {}", ind, cursor.col);
                        }
                    }
                } else {
                    let mut nt = "";

                    //probably chars would be better, also it is better to put them in vec?
                    nt = match kev.scan_code {
                        2 => "1",
                        3 => "2",
                        4 => "3",
                        5 => "4",
                        6 => "5",
                        7 => "6",
                        8 => "7",
                        9 => "8",
                        10 => "9",
                        11 => "0",
                        16 => "'",
                        17 => ",",
                        18 => ".",
                        19 => "p",
                        20 => "y",
                        21 => "f",
                        22 => "g",
                        23 => "c",
                        24 => "r",
                        25 => "l",
                        26 => "/",
                        27 => "=",
                        28 => "\n", //will create a new section in the future
                        30 => "a",
                        31 => "o",
                        32 => "e",
                        33 => "u",
                        34 => "i",
                        35 => "d",
                        36 => "h",
                        37 => "t",
                        38 => "n",
                        39 => "s",
                        40 => "-",
                        43 => "\\",
                        44 => ";",
                        45 => "q",
                        46 => "j",
                        47 => "k",
                        48 => "x",
                        49 => "b",
                        50 => "m",
                        51 => "w",
                        52 => "v",
                        53 => "z",
                        57 => " ",
                        105 => {
                            if cursor.col > 0 {
                                cursor.col -= 1; //probably possible to update ind iteratively relying on the current valid char
                                cursor.ind = None;
                            }
                            ""
                        }
                        106 => {
                            cursor.col += 1;
                            cursor.ind = None;
                            ""
                        }
                        sc => "",
                    };
                    //updates the ind so it safely fits chars, also updates cursor.col to fit the line
                    if cursor.ind.is_none() {
                        cursor.ind = Some(0);
                        let mut colmax = 0;
                        for (n, (i, c)) in text.sections[0].value.char_indices().enumerate() {
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

                    text.sections[0]
                        .value
                        .insert_str(cursor.ind.unwrap_or(0), nt);
                    if let Some(i) = &mut cursor.ind {
                        let l = nt.len();
                        *i += l; //updates the ind
                        if l > 0 {
                            cursor.col += 1; //char printed, update the col
                        }
                    }
                    println!("cursor: col {}, ind {:?}", cursor.col, cursor.ind);

                    //text.sections[0].value.push_str(&format!("{}", nt));
                }
            }
        }

        transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
    }
}
