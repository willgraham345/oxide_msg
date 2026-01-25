Schema relationships
- message_name
- type
  - LowLevel
    - CCSDS
    - RS422
    - File descriptor / syscall
  - Network msg
    - TCP
    - UDP
- data --> Structure of the data you're implementing
- type_impl --> Type-specific flags and/or management 
- security
  - allow_list
  - deny_list
- owner / server

Optionals:
- hardware_interface
