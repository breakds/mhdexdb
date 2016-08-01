// Module content and metadata are private, whose members are
// selectively exposed.
mod content;
pub use self::content::Weapon;

mod metadata;
pub use self::metadata::SharpnessColor;
pub use self::metadata::WeaponColumn;
pub use self::metadata::WeaponType;



