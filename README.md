# Yew-projects

A collection of tools to better flesh out the Rust frontend story with Yew.

## The projects
----

| Project      | Info                                                                        |
| ------------ | --------------------------------------------------------------------------- |
| yew-pack     | CLI tool for developing, building, testing, and publishing yew sites        |
| yew-recoil   | Port of RecoilJS to yew for easy, async, atom-based state management        |
| yew-store    | Simple port of Redux Toolkit for dispatch/selector/reduce state management  |
| yew-dnd      | Drag-and-drop library for Yew applications                                  |
| yew-three    | 3D renderer system similar to react-three-fiber for reactive 3D apps        |
| yew-joint    | SVG + DOM based canvas system for grid-style apps                           |
| yew-splitter | Port of react-splitter to Yew                                               |
| yew-suspense | Simple wrappers around async values in Yew components                       |
| yew-rocket   | Yew integration with Rocket for simple serving and rehydration of Yew sites |
| yew-desktop  | CLI tool + services to ship a Yew site as a desktop application             |


## Good examples
----

| Example            | Info                                       |
| ------------------ | ------------------------------------------ |
| yew-pack-example   | Simple template with yew-pack instructions |
| yew-recoil-example | Fairly complex demonstration of yew-recoil |


## Other good tools

| Project    | Info                        |
| ---------- | --------------------------- |
| yew-router | Flexible macro-based router |



<!-- Hey,

I'm working exactly on this, but instead of Redux, I'm working on porting RecoilJS. You need to wrap your app in a context, or write a custom context. There are context examples in the functional crate, but feel free to use my context and hook as inspiration. Currently, my atoms don't memoize their inputs/outputs which is a key feature of selectors, so you'll want to implement the memoization and suppress downstream updates to selector subscriptions. Redux's model is fairly straightforward.

https://github.com/jkelleyrtp/yew-projects/blob/master/packages/yew-recoil/src/context.rs
https://github.com/jkelleyrtp/yew-projects/blob/master/packages/yew-recoil/src/hooks.rs

If you want to contribute, I'm also using the same learnings from my RecoilJS implementation 
 -->
