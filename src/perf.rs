use gl;

pub enum GraphRenderStyle {
    FPS,
    MS,
    PERCENT
}

pub struct PerfGraph<'a> {
    style: GraphRenderStyle,
    name: &'a str,
    values: Vec<f32>,
    //head
}

impl<'a> PerfGraph<'a> {
    pub fn get_average(&self) -> f32 {
        //
        // TODO : get the code here
        //
        0.0
    }

    pub fn new(style: GraphRenderStyle, name: &'a str) -> PerfGraph {
        //
        // TODO : finish initGraph
        //
        PerfGraph {
            style,
            name,
            values: vec!()
        }
    }

    pub fn render(&self) {
        //
        // TODO :get the code here
        //
    }

    pub fn update(&self, frame_time: f32) {
        //
        // TODO : get the code here
        //
    }
}

pub struct GPUTimer {
    //
    // TODO : what to do here?
    //
    cur: usize,
    ret: usize
}

impl GPUTimer {
    pub fn new() -> GPUTimer { GPUTimer { cur: 0, ret: 0} }

    pub fn start(&self) {
        //
        // TODO : get the code here
        //
        //unsafe {
        //    gl::BeginQuery(gl::GL_E
        //
    }

    pub fn stop(&self) {
        //
        // TODO : get the code here
        //
    }
}

