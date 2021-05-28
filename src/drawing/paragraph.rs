use nvg::Context;
use nvg_gl::Renderer;

const _PARA_TEXT: &str = "This is longer chunk of text.\n  \n  Would have used lorem ipsum but she    was busy jumping over the lazy dog with the fox and all the men who came to the aid of the party.🎉";
const _PARA_HOVER_TEXT: &str = "Hover your mouse over the text to see calculated caret position.";
const _PARA_BOX_TEXT: &str = "Testing\nsome multiline\ntext.";

pub fn draw(
    mut ctx: Context<Renderer>,
    _x: f32, _y: f32, _w: f32, _h: f32, _mx: f32, _my: f32
) -> Context<Renderer> {
    ctx.save();
    ctx.font_size(15.0);
    ctx.font("sans");
    ctx.text_align(nvg::Align::LEFT | nvg::Align::TOP);
    //
    let _lineh = ctx.text_metrics();
    //
    //
    //
    //
    //
    ctx.restore();
    ctx
}