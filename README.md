# LD44 - Your life is currency 

Unfinished game project for the *Ludum Dare 44* - challenged myself to make a Rust game working in the browser via WASM, while trying out Specs and Quicksilver.

Gave up because *ATM* creating UIs with Quicksilver is purely unfeasable and I lost too much time :(

Uploaded on github to provide a "working" example of : 
* Using an [ECS](https://en.wikipedia.org/wiki/Entity_component_system) pattern (Specs) to implement a turn-based puzzle game (even though my design may require some work to be usable in a bigger scope) 
* Experiment on Pathfinding in a tile-based environment using an ECS `Resource`   
* Displaying things on the screen with Specs + Quicksilver (see `renderer.rs`) 
* Experiment with multi-state management in Quicksilver (or lack, thereof)
* Async asset management (rushed a bit)

More info about the (unfinished) game concept and my other ideas in `ideas.md`