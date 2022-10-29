use std::f32::consts::PI;

pub fn to_radians(deg:f32) -> f32 {
    deg*PI/180.0
}
pub fn to_degrees(rads:f32) -> f32 {
    rads/PI*180.0
}

pub struct Camera2D {
    pub width:f32,
    pub height:f32,
    pub proj:glam::Mat4,
    pub view:glam::Mat4,
    pub model:glam::Mat4,
    pub z_far:f32,
    pub z_near:f32,
    a:f32,
    b:f32,
    scale:f32
}


impl Camera2D {
    pub fn new(mut width:f32,mut height:f32,z_far:f32,z_near:f32,scale:f32) -> Camera2D {
        width = width/scale;
        height = height/scale;     
        let proj = glam::Mat4::orthographic_rh_gl(-width/2.0, width/2.0, -height/2.0, height/2.0, z_near, z_far);

        Camera2D {
            width,
            height,
            proj,
            //proj:glam::Mat4::perspective_rh_gl(to_radians(fov), width/height, z_near, z_far),
            view:glam::Mat4::from_translation(glam::vec3(0.0,0.0,-1.0)),
            model:glam::Mat4::from_translation(glam::vec3(0.0,0.0,0.0)),
            z_far:z_far,
            z_near:z_near,
            a:proj.x_axis[0],
            b:proj.y_axis[1],
            scale
        }
    }

    pub fn update_projection(&mut self,width:f32,height:f32) {
        self.width  = width/self.scale;
        self.height = height/self.scale;
        let a = 2.0 / (self.width/2.0 +  self.width/2.0);
        let b = 2.0 / (self.height/2.0 + self.height/2.0);
        let a_scale = self.proj.x_axis[0]/self.a;
        let b_scale = self.proj.y_axis[1]/self.b;
        self.a = a;
        self.b = b;
        self.proj.x_axis[0] = a_scale*a;
        self.proj.y_axis[1] = b_scale*b;
    }
    #[inline]
    pub fn move_left(&mut self, delta:f32) {
        self.view.w_axis[0] += delta;
    }
    #[inline]
    pub fn move_right(&mut self, delta:f32) {
        self.view.w_axis[0] -= delta;
    }
    #[inline]
    pub fn move_up(&mut self, delta:f32) {
        self.view.w_axis[1] -= delta;
    }
    #[inline]
    pub fn move_down(&mut self, delta:f32) {
        self.view.w_axis[1] += delta;
    }
    #[inline]
    pub fn zoom_in(&mut self, delta:f32) {
        self.proj.x_axis[0] *= 1.0+delta*2.0;
        self.proj.y_axis[1] *= 1.0+delta*2.0;

    }
    #[inline]
    pub fn zoom_out(&mut self, delta:f32) {
        self.proj.x_axis[0] *= 1.0-delta*2.0;
        self.proj.y_axis[1] *= 1.0-delta*2.0;
    }
    #[inline]
    pub fn scale_glfw_cursor(&self,xpos:&mut f64,ypos:&mut f64) {
        *xpos /= self.width as  f64*self.scale as f64 * (self.proj.x_axis[0]) as f64;
        *ypos /= self.height as f64*self.scale as f64 * (self.proj.y_axis[1]) as f64;
        *ypos = 1.0-*ypos;

        *xpos -= (1.0-self.proj.x_axis[0] as f64)/(self.proj.x_axis[0]*2.0) as f64;
        *ypos += (1.0-self.proj.y_axis[1] as f64)/(self.proj.y_axis[1]*2.0) as f64;

        *xpos -= self.view.w_axis[0] as f64/(2.0) as f64;
        *ypos -= self.view.w_axis[1] as f64/(2.0) as f64;   
    }

}





