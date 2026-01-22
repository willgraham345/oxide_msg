# Oxide_msg
This repo is me exploring if there is an easier way to write and think about messaging. The implementation is looking into how I can write a framework which will act as something similar to the [uv](https://docs.astral.sh/uv/) of the messaging world. 

Things I'm trying to do:
- Abstract *part* of [serde](https://serde.rs/) away. Get away from "i define everything with this trait" and move towards configuration as code
- Heavily test each message. Add a framework to quickly and easily write unit tests for a group of messages. 
- Provide a TUI interface for sending, receiving, and inspecting various messages. 

To see a more thorough explanation, see the project [Obsidian Publish Docs](https://github.com/rust-lang/mdBook). Why did I do it this way? Because I like Obsidian, and I didn't want to learn another stupid tool.
