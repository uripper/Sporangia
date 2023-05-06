use std::time::Instant;
slint::slint!(
    export component Splash inherits Window{
   no-frame: true;
   background: rgba(0, 0, 0, 1.0);
   Image{
       opacity: 1.0;
       source: @image-url("src/ui/assets/Sporangia.png");
       animate opacity {
           duration: 3s;
        }
    }
}
);


pub fn run() {
    main()
}

fn main() {
    let splash = Splash::new().unwrap();
    let now = Instant::now();
    display_splash(now, splash);
}

fn display_splash(now: Instant, splash: Splash){
    splash.show().unwrap();
    loop{
        if now.elapsed().as_millis() < 10000 {
        let _ = slint::quit_event_loop();
        let _ = slint::run_event_loop();
    }
    else{
        splash.hide().unwrap();
        break;
    }
}
}


