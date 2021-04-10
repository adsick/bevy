//keyboard abstraction layer
use bevy::input::keyboard::KeyboardInput;
use bevy::{
    input::ElementState,
    utils::{HashMap, StableHashSet},
};
#[derive(Debug)]
pub struct KeyStrokes {
    strokes: HashMap<u32, Stroke>,
    releases: HashMap<u32, Release>,
}

impl KeyStrokes {
    pub fn new() -> Self {
        KeyStrokes {
            strokes: HashMap::default(),
            releases: HashMap::default(),
        }
    }

    pub fn get_just_pressed(&self) -> StableHashSet<u32> {
        //change this to return one value?
        // very likely to be just one stroke
        let mut res = StableHashSet::default();

        for (sc, stroke) in &self.strokes {
            if stroke.duration == 0.0 {
                res.insert(*sc);
            }
        }
        res
    }
    pub fn get_down(&self) -> StableHashSet<u32> {
        //does not preserve the order
        //future sorted version of this
        let mut res = StableHashSet::default();

        for (sc, stroke) in &self.strokes {
            // if stroke.duration > crate::DOWN_INTERVAL {
            //     res.insert(*sc);
            // }
            res.insert(*sc);
        }
        res
    }
    // pub fn get_active(&self)-> StableHashSet<u32> {
    //     let mut res = StableHashSet::default();

    //     for (sc, _) in &self.strokes {
    //             res.insert(*sc);
    //     }
    //     res
    // }

    pub fn get_double_pressed(&self) -> StableHashSet<u32> {
        let mut res = StableHashSet::default();

        for (sc, stroke) in &self.strokes {
            if stroke.duration == 0.0 && stroke.repetition == 1 {
                res.insert(*sc);
            }
        }
        res
    }

    pub fn eat_keyboard_input(&mut self, kev: &KeyboardInput) {
        let sc = kev.scan_code;
        let est = kev.state;

        let strokes = &mut self.strokes;
        let releases = &mut self.releases;

        if est == ElementState::Pressed {
            if let None = strokes.get_mut(&sc) {
                if let Some(release) = releases.remove(&sc) {
                    if release.duration < crate::REPEAT_INTERVAL {
                        strokes.insert(
                            sc,
                            Stroke {
                                duration: 0.0,
                                repetition: release.repetition + 1,
                            },
                        );
                    } else {
                        strokes.insert(
                            sc,
                            Stroke {
                                duration: 0.0,
                                repetition: 0,
                            },
                        );
                    }
                } else {
                    //add nonexistent stroke to strokes
                    strokes.insert(
                        sc,
                        Stroke {
                            duration: 0.0,
                            repetition: 0,
                        },
                    );
                }
            }
        } else {
            //don't forget to remove from releases
            if let Some(stroke) = strokes.remove(&sc) {
                //key exists

                releases.insert(
                    sc,
                    Release {
                        duration: 0.0,
                        repetition: stroke.repetition,
                    },
                );
            } else {
                //add nonexistent release to releases
                releases.insert(
                    sc,
                    Release {
                        duration: 0.0,
                        repetition: 0,
                    },
                );
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.strokes
            .values_mut()
            .for_each(|stroke| stroke.duration += dt);
        self.releases
            .values_mut()
            .for_each(|release| release.duration += dt);
    }
}

#[derive(Debug)]
pub struct Stroke {
    pub duration: f32,
    repetition: u16,
}
#[derive(Debug)]
pub struct Release {
    pub duration: f32,
    repetition: u16,
}
