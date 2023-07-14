use sdl2::EventPump;

use crate::constants::{MOUSE_SENSIVITY, HEIGHT, WIDTH};

pub struct EventControls {
    player_pos: [f32; 3],
    player_angle: [f32; 3]
}


impl EventControls {

    // TODO FAIRE LES EVENT INPUT CLAVIER SOURIS

    /// initialisation de la caméra
    /// [x,y,z] position caméra / joueur
    pub fn init(x: f32, y: f32, z: f32) -> EventControls
    {
        return EventControls {
            player_pos: [x, y, z],
            player_angle: [0., 0., 0.]
        };
    }

    pub fn get_pos_player(&self) -> [f32; 3]
    {
        return self.player_pos;
    }

    pub fn get_angle_player(&self) -> [f32; 3]
    {
        return self.player_angle;
    }

    pub fn controls(&mut self , event_pump: &mut EventPump, _sdl_context: &mut sdl2::Sdl) -> bool
    {
        // si il n'y pas d'event, osef
        if event_pump.poll_event().is_none()
        {
            return false;
        }


        // MOUSE 
        // inversion x,y -> y,x car les axes de la souris ne sont pas les mêmes que ceux de l'écran
        self.player_angle = [event_pump.mouse_state().y() as f32 / HEIGHT * MOUSE_SENSIVITY, event_pump.mouse_state().x() as f32 / WIDTH * MOUSE_SENSIVITY, 0.];

        // END MOUSE



        let keyboard_events = event_pump.keyboard_state();

        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::W)
        {
            self.player_pos[2] += - 1.;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::S)
        {
            self.player_pos[2] += 1.;
        }


        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::A)
        {
            self.player_pos[0] += 1.;
        }
        else if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::D)
        {
            self.player_pos[0] += -1.;
        }


        //  quitter le programme
        if keyboard_events.is_scancode_pressed(sdl2::keyboard::Scancode::Escape)
        {
            return true;
        }



        return false;
    }




}