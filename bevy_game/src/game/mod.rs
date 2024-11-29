use bevy::prelude::*;


pub mod level;

pub fn plugin(app:&mut App){
    //TODD -demo -
    app.add_plugins(level::plugin);
}