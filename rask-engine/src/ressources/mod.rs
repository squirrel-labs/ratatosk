/*!
The ressource management system for the ratatosk game engine
*/

pub mod libary;
pub mod sound;
pub mod texture;

enum Ressource {
    Texture(texture::Texture),
    Sound(sound::Sound),
    Skeleton(spine::skeleton::Skeleton),
}
