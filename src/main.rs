mod brainpower;

use brainpower::BrainPower;

fn main() {
    let bp = r#"
Are you ready?
Are you ready?

Adrenaline is pumping
Adrenaline is pumping
Generator. Automatic lover
Atomic. Atomic. Overdrive
Blockbuster. Brain power
Call me a leader. Cocaine
Don't you try it, don't you try it
Innovator. Killer machine
There's no fate. Take control
Brain power

Let the bass kick!
OJ-IO-IUJ-II-JJJ EJ-eJAo-JOU IIU-EEEEEEEE--Oo E-EEE---OoEEoEEE----
oEEE-oE AAAA---eUo-Eo-Eo-eoo EOAU-Ae-Uoo-I-o-eee I--EEEEEEE----
II-EEE I-oo I-Ae-IAI-EEE I-eeeeee I-eeeeeeee I-oo EIo-EEI----
"#;

    println!("Your song:\n{}\n", bp);
    println!("let's run it! it say:\n\n");

    let mut bp = BrainPower::new(bp);
    bp.run();

    println!("\n\nYou Brain Powered!");
}
