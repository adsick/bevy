pub enum Layout {
    Dvorak,
    Rus,
}



impl Layout {
    pub fn scch(&self, sc: u32) -> char {
        match *self {
            Self::Dvorak => Self::scdv(sc),
            Self::Rus => Self::scru(sc),
        }
    }

    pub fn switch(&mut self) {
        if let Self::Dvorak = self {
            *self = Self::Rus;
        } else {
            *self = Self::Dvorak;
        }
    }

    fn scdv(sc: u32) -> char {
        //scancode to dvorak converter
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

    fn scru(sc: u32) -> char {
        //scancode to dvorak converter
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
            16 => 'й',
            17 => 'ц',
            18 => 'у',
            19 => 'к',
            20 => 'е',
            21 => 'н',
            22 => 'г',
            23 => 'ш',
            24 => 'щ',
            25 => 'з',
            26 => 'х',
            27 => 'ъ',
            28 => '\n', //will create a new section in the future
            30 => 'ф',
            31 => 'ы',
            32 => 'в',
            33 => 'а',
            34 => 'п',
            35 => 'р',
            36 => 'о',
            37 => 'л',
            38 => 'д',
            39 => 'ж',
            40 => 'э',
            43 => '\\',
            44 => 'я',
            45 => 'ч',
            46 => 'с',
            47 => 'м',
            48 => 'и',
            49 => 'т',
            50 => 'ь',
            51 => 'б',
            52 => 'ю',
            53 => '.',
            57 => ' ',
            _ => '\0',
        }
    }
}
