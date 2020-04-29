extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct RpnCalc {

    #[nwg_control(size: (300, 750), position: (100, 100), title: "RpnCalc")]
    #[nwg_events( OnWindowClose: [RpnCalc::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 2, min_size: [150, 140])]
    grid: nwg::GridLayout,

    #[nwg_control(text: "", align: nwg::HTextAlign::Right, readonly: true)]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 4, col_span: 4)]
    input: nwg::TextInput,

    #[nwg_control(text: "drop")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 4)]
    #[nwg_events( OnButtonClick: [RpnCalc::drop] )]
    btn_drop: nwg::Button,

    #[nwg_control(text: "swap")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 4)]
    #[nwg_events( OnButtonClick: [RpnCalc::swap] )]
    btn_swap: nwg::Button,

    #[nwg_control(text: "roll")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 4)]
    #[nwg_events( OnButtonClick: [RpnCalc::roll] )]
    btn_roll: nwg::Button,

    #[nwg_control(text: "eex")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 4)]
    #[nwg_events( OnButtonClick: [RpnCalc::eex] )]
    btn_eex: nwg::Button,

    #[nwg_control(text: "x^2")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 5)]
    #[nwg_events( OnButtonClick: [RpnCalc::sqr] )]
    btn_sqr: nwg::Button,

    #[nwg_control(text: "sqrt")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 5)]
    #[nwg_events( OnButtonClick: [RpnCalc::sqrt] )]
    btn_sqrt: nwg::Button,

    #[nwg_control(text: "x^y")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 5)]
    #[nwg_events( OnButtonClick: [RpnCalc::pow] )]
    btn_pow: nwg::Button,

    #[nwg_control(text: "1/x")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 5)]
    #[nwg_events( OnButtonClick: [RpnCalc::inv] )]
    btn_inv: nwg::Button,

    #[nwg_control(text: "enter")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 6, col_span: 2)]
    #[nwg_events( OnButtonClick: [RpnCalc::enter] )]
    btn_enter: nwg::Button,

    #[nwg_control(text: "<-")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 6)]
    #[nwg_events( OnButtonClick: [RpnCalc::back] )]
    btn_back: nwg::Button,

    #[nwg_control(text: "mod")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 6)]
    #[nwg_events( OnButtonClick: [RpnCalc::rem] )]
    btn_mod: nwg::Button,

    #[nwg_control(text: "7")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 7)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_7: nwg::Button,

    #[nwg_control(text: "8")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 7)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_8: nwg::Button,

    #[nwg_control(text: "9")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 7)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_9: nwg::Button,

    #[nwg_control(text: "/")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 7)]
    #[nwg_events( OnButtonClick: [RpnCalc::div] )]
    btn_div: nwg::Button,

    #[nwg_control(text: "4")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 8)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_4: nwg::Button,

    #[nwg_control(text: "5")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 8)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_5: nwg::Button,

    #[nwg_control(text: "6")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 8)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_6: nwg::Button,

    #[nwg_control(text: "*")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 8)]
    #[nwg_events( OnButtonClick: [RpnCalc::mult] )]
    btn_mult: nwg::Button,

    #[nwg_control(text: "1")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 9)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_1: nwg::Button,

    #[nwg_control(text: "2")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 9)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_2: nwg::Button,

    #[nwg_control(text: "3")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 9)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_3: nwg::Button,

    #[nwg_control(text: "-")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 9)]
    #[nwg_events( OnButtonClick: [RpnCalc::sub] )]
    btn_sub: nwg::Button,

    #[nwg_control(text: "0")] 
    #[nwg_layout_item(layout: grid, col: 0, row: 10)]
    #[nwg_events( OnButtonClick: [RpnCalc::number(SELF, CTRL)] )]
    btn_0: nwg::Button,

    #[nwg_control(text: ".")] 
    #[nwg_layout_item(layout: grid, col: 1, row: 10)]
    #[nwg_events( OnButtonClick: [RpnCalc::dot] )]
    btn_dot: nwg::Button,

    #[nwg_control(text: "+/-")] 
    #[nwg_layout_item(layout: grid, col: 2, row: 10)]
    #[nwg_events( OnButtonClick: [RpnCalc::chs] )]
    btn_chs: nwg::Button,

    #[nwg_control(text: "+")] 
    #[nwg_layout_item(layout: grid, col: 3, row: 10)]
    #[nwg_events( OnButtonClick: [RpnCalc::add] )]
    btn_add: nwg::Button,

}

impl RpnCalc {

    fn number(&self, button: &nwg::Button) {
        let text = self.input.text();
        self.input.set_text(&format!("{}{}", text, button.text()));
    }

    fn dot(&self) {        
    }

    fn chs(&self) {        
    }

    fn add(&self) {        
    }

    fn sub(&self) {        
    }

    fn mult(&self) {        
    }

    fn div(&self) {        
    }

    fn rem(&self) {        
    }

    fn enter(&self) {        
    }

    fn back(&self) {        
    }

    fn drop(&self) {
        self.input.set_text("");
    }

    fn swap(&self) {        
    }

    fn roll(&self) {        
    }

    fn eex(&self) {        
    }

    fn sqr(&self) {        
    }

    fn sqrt(&self) {        
    }

    fn pow(&self) {        
    }

    fn inv(&self) {        
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}


fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    RpnCalc::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
