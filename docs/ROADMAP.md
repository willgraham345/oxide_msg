To summarize
- [[CS Messaging and Serialization]]
	- [[Networking Protocols]]
	- [[Networking Messaging]]

- Needs
  - cmd / tlm definitions 
  - Silos
  - GUID / PID values

- Messages supported
  - Serde messages
    - Defined within yaml, or potentially a conversion script...
  - File descriptors for processes syscalls
    - [[Linux syscall read]]
    - Linux syscall 
    - Linux send
  - Low level messaging (start with these, move onto other frameworks)
    - CCSDS
    - Spacewire 
    - RS422
    - Ethernet...?
    - CAN
    - Alternative messaging frameworks
      - Protobuf
      - Flatbuffers
      - MQT
  - Networking I/O
    - iptables
    - nftables
    - VXLAN / GRE (not sure what these are)
    - TCP
    - Ethernet...?
- CLI Tool
  - Output messages in a buffer, or to a log-file
  - Graphing tlm values with timestamp
  - Graphing cmd values, and state changes...?
  - Live monitoring of configuration values
    - These might need to have additional info given with command information
- Scripting support? (i.e. testing framework)

Low Priority:
- Bluetooth messages?
- Additional syscall support (i.e. scripting / protocol management)
