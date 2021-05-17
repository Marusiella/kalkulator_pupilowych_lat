use gtk::prelude::{BuilderExtManual};
use gtk::{
    ButtonExt, EntryExt, LabelExt, SpinButtonExt, SwitchExt,
    ToggleButtonExt, WidgetExt,
};

fn main() {
    gtk::init().unwrap();
    let builder = gtk::Builder::from_file("gtk.glade");
    let window: gtk::Window = builder.get_object("window").unwrap();
    let spinbutton: gtk::SpinButton = builder.get_object("spinbutton").unwrap();
    let switch: gtk::Switch = builder.get_object("switch").unwrap();
    // pies
    let label1: gtk::Label = builder.get_object("label5").unwrap();
    let label4: gtk::Label = builder.get_object("label4").unwrap();
    let chec1: gtk::CheckButton = builder.get_object("check1").unwrap();
    let chec2: gtk::CheckButton = builder.get_object("check2").unwrap();
    let chec3: gtk::CheckButton = builder.get_object("check3").unwrap();
    let oblicz: gtk::Button = builder.get_object("oblicz").unwrap();

    spinbutton.set_range(0f64, 25f64);
    spinbutton.set_numeric(true);
    spinbutton.set_increments(1f64, 1f64);
    oblicz.set_label("Oblicz");

    oblicz.connect_clicked(move |_| {
        if switch.get_active() {
            // pies
            &mut label1.show();
            &mut chec1.show();
            &mut chec2.show();
            &mut chec3.show();
            let mut ttt = 222;
            let mut x = true;
            // println!("1{} 2{} 3{}", chec1.get_active(), chec2.get_active(), chec3.get_active());
            if chec1.get_active() == true
                && chec2.get_active() == false
                && chec3.get_active() == false
            {
                ttt = 0;
                spinbutton.set_range(0f64, 20f64);

            } else if chec1.get_active() == false
                && chec2.get_active() == true
                && chec3.get_active() == false
            {
                ttt = 1;
                spinbutton.set_range(0f64, 18f64);

            } else if chec1.get_active() == false
                && chec2.get_active() == false
                && chec3.get_active() == true
            {
                ttt = 2;
                spinbutton.set_range(0f64, 17f64);

            } else {

                label4.set_label(&*format!(
                    "ustaw poprawnie wielkość {}",
                    if switch.get_active() { "psa" } else { "kota" }
                ));
                x = false;
            }
            if ttt == 222 {
                label4.set_label(&*format!(
                    "ustaw poprawnie wielkość {}",
                    if switch.get_active() { "psa" } else { "kota" }
                ));
                x = false;
            }
            if x && spinbutton.get_text().parse::<u8>().unwrap() > 0u8 {
                label4.set_label(&*format!(
                    "Wiek twojego pupila na ludzkie lata to: {} lat",
                    pies(ttt, spinbutton.get_text().parse::<u8>().unwrap())
                ));
            } else if  spinbutton.get_text().parse::<u8>().unwrap() == 0u8{
                label4.set_label(&*format!(
                    "ustaw poprawnie wiek {}",
                    if switch.get_active() { "psa" } else { "kota" }
                ));
            }
            // println!("{}", ttt);
        } else {
            //kot
            &mut label1.hide();
            &mut chec1.hide();
            &mut chec2.hide();
            &mut chec3.hide();
            spinbutton.set_range(0f64, 25f64);
            if spinbutton.get_text().parse::<u8>().unwrap() == 0u8{
                label4.set_label(&*format!(
                    "ustaw poprawnie wiek {}",
                    if switch.get_active() { "psa" } else { "kota" }
                ));
            } else {
                label4.set_label(&*format!(
                    "Wiek twojego pupila na ludzkie lata to: {} lat",
                    kot( spinbutton.get_text().parse::<u8>().unwrap())
                ));
            }

        }
    });
    window.connect_destroy(|_| {
        gtk::main_quit();
    });


    window.show_all();
    gtk::main();
}

fn pies(typ_pasa: u8, lata: u8) -> u32 {
    match typ_pasa {
        0 => {
            return match lata {
                1 => 20,
                2 => 30,
                3 => 33,
                4 => 36,
                5 => 40,
                6 => 44,
                7 => 48,
                8 => 50,
                9 => 55,
                10 => 58,
                11 => 63,
                12 => 67,
                13 => 71,
                15 => 78,
                16 => 83,
                17 => 87,
                18 => 90,
                19 => 95,
                20 => 98,
                _ => 0,
            }
        }
        1 => {
            return match lata {
                1 => 18,
                2 => 28,
                3 => 35,
                4 => 40,
                5 => 45,
                6 => 52,
                7 => 58,
                8 => 63,
                9 => 69,
                10 => 75,
                11 => 80,
                12 => 87,
                13 => 92,
                14 => 98,
                15 => 103,
                16 => 109,
                17 => 115,
                18 => 120,
                _ => 0,
            }
        }
        2 => {
            return match lata {
                1 => 12,
                2 => 22,
                3 => 31,
                4 => 41,
                5 => 48,
                6 => 55,
                7 => 62,
                8 => 68,
                9 => 75,
                10 => 82,
                11 => 89,
                12 => 97,
                13 => 103,
                14 => 110,
                15 => 118,
                16 => 123,
                17 => 130,
                _ => 0,
            }
        }

        _ => 12,
    }
}

// radio1 mała == 0
// radio2 średnia = 1
// radio3 duża = 2


fn kot(lata: u8) -> u32 {
    match lata {
        1 => 15,
        2 => 24,
        3 => 28,
        4 => 32,
        5 => 36,
        6 => 40,
        7 => 44,
        8 => 48,
        9 => 52,
        10 => 56,
        11 => 60,
        12 => 64,
        13 => 68,
        14 => 72,
        15 => 76,
        16 => 80,
        17 => 84,
        18 => 88,
        19 => 92,
        20 => 96,
        21 => 100,
        22 => 104,
        23 => 108,
        24 => 112,
        25 => 116,
        _ => 0,
    }
}