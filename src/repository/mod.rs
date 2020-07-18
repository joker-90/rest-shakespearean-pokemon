use crate::repository::pokemon::PokemonApiRepository;
use crate::repository::translator::TranslatorApiRepository;

pub mod translator;
pub mod pokemon;

pub struct Repositories {
    pub translator: TranslatorApiRepository,
    pub pokemon: PokemonApiRepository,
}

impl Repositories {
    pub fn default() -> Repositories {
        Repositories {
            translator: TranslatorApiRepository::default(),
            pokemon: PokemonApiRepository::default(),
        }
    }
}