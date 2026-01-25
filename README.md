# Oxide_msg
This repo is me exploring if there is an easier way to write and think about messaging. The implementation is looking into how I can write a framework which will act as something similar to the [uv](https://docs.astral.sh/uv/) of the messaging world. 

Things I'm trying to do:
- Abstract *part* of [serde](https://serde.rs/) away. Get away from "i define everything with this trait, and I write it all down in my own code" and move towards configuration that generates workable code. I.e. you define configuration, run a command, and autocode is generated that **YOU** can edit. 
  - This provides methods for generating code through macros, as well as code through rust-lang 
  - Add methods for enabling custom types, and widely supported types. For a better idea of where this is, see the [ROADMAP](./docs/ROADMAP.md)
- Heavily test each message. Add a framework to quickly and easily write unit tests for a group of messages. 
- Provide a TUI interface for sending, receiving, and inspecting various messages. 

To see a more thorough explanation, see the project [Obsidian Publish Docs](https://github.com/rust-lang/mdBook). Why did I do it this way? Because I like Obsidian, and I didn't want to learn another stupid tool.

# Open questions
How should I have configuration handle specific flags?
- Could I have each struct add additional unset fields, and have the user define those fields as needed?
