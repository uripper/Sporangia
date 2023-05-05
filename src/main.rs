slint::slint!{
    import { Button, VerticalBox }  from "std-widgets.slint";
    import "src/ui/fonts/Manrope-ExtraBold.otf";
    import "src/ui/fonts/contm.ttf";
    import {Palette} from "src/ui/components/palette.slint";
    import {Menu} from "src/ui/widgets/menu.slint";
    import {Splash} from "src/ui/components/splash.slint";

    export component App inherits Window{
        title: "Sporangia";
        width: 800px;
        height: 600px;
        background: Palette.cherise;
        property <string> state: "splash";
        if(state == "splash") : Splash{}
}
}
fn main() {
    let app = App::new().unwrap();
    let app_weak = app.as_weak();
    app.run().unwrap();
}
