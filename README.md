# txtrpg-rs
Welcome traveler! Don't be frightened by that *cryptc* name above. This project aims to implement the **very basic** data and mechanics of an RPG. Of course, there are a lot of ways to interpret and implement an RPG and I guess there will always be someone who disagrees with my implementation. So feel free to contribute and share the code and if this doesn't feel right to you, create a fork and create your own better variant.

## Roadmap
- [ ] own, better name generation
- [X] make `item_generator` its own struct with a builder pattern for generating items
- [ ] implement game-worlds (2d grid based)
- [ ] implement game-world loading from file
- [ ] implement level/campagne/world scripting through a derived ASM dialect
- [ ] make output interface more common and move everything into a crate
- [X] adapt `influence` from items to `ItemGenerator`
- [ ] add `requirement` to `Item`
- [ ] make item name and attribute generation more fine-grained, related to the item's type
- [ ] model out entites
- [ ] create ECS for NPCs
- [ ] create dialog system
- [ ] create event system