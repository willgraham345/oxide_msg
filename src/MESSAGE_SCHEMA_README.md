# Message Documentation Schema

This directory contains a JSON Schema for documenting messages in the Oxide messaging framework.

## Files

- **message_schema.json** - JSON Schema definition for YAML message documentation
- **message_example.yaml** - Example YAML file demonstrating schema usage

## Schema Overview

The message documentation schema defines the following fields:

### Required Fields

- **message_name** (string): The name of the message
- **protocol** (string): The protocol used for message transmission (e.g., "tcp", "udp", "ipc")
- **packetizer** (string): The packetizer method used for message serialization (e.g., "json", "msgpack", "protobuf")
- **base_node** (string): The base node that originates or manages this message
- **whitelist_nodes** (array): List of nodes explicitly allowed to receive or send this message
- **blacklist_nodes** (array): List of nodes explicitly denied from receiving or sending this message
- **server** (string): The server address or identifier for this message
- **message_type** (string): The type or pattern of the message ("pubsub", "reqrep", "pipeline", or "custom")

### Optional Fields

- **hardware_interface** (string): Hardware interface specification for the message (e.g., "eth0", "can0", "serial0")

## Usage

### Creating a Message Documentation File

1. Create a new YAML file in your project
2. Follow the structure defined in `message_example.yaml`
3. Validate your YAML file against `message_schema.json`

### Example

```yaml
message_name: "sensor_telemetry"
protocol: "tcp"
packetizer: "json"
hardware_interface: "eth0"  # Optional
base_node: "sensor_controller"
whitelist_nodes:
  - "telemetry_processor"
  - "data_logger"
blacklist_nodes:
  - "legacy_system"
server: "tcp://127.0.0.1:5555"
message_type: "pubsub"
```

### Validating YAML Against Schema

You can validate your YAML message documentation files using various tools:

#### Using Python with jsonschema and pyyaml

```bash
pip install jsonschema pyyaml
```

```python
import json
import yaml
from jsonschema import validate

# Load the schema
with open('src/message_schema.json', 'r') as f:
    schema = json.load(f)

# Load and validate a YAML file
with open('your_message.yaml', 'r') as f:
    message_doc = yaml.safe_load(f)

validate(instance=message_doc, schema=schema)
print("Valid!")
```

#### Using Online Validators

1. Convert your YAML to JSON using an online converter
2. Validate against the schema at https://www.jsonschemavalidator.net/

## Message Types

The schema supports the following message types that align with Oxide's messaging patterns:

- **pubsub**: Publisher/Subscriber pattern - one-to-many message distribution
- **reqrep**: Request/Reply pattern - synchronous request-response communication
- **pipeline**: Push/Pull pattern - distributed task processing
- **custom**: Custom messaging pattern not covered by the above

## Schema Compliance

The schema follows JSON Schema Draft 07 specification and can be used with any JSON Schema validator.

## Integration with Oxide Framework

This schema is designed to document messages used in the Oxide messaging framework. While the framework uses JSON for runtime message payloads, this YAML schema provides a standardized way to document message configurations, routing rules, and metadata.

For runtime message structure, refer to the `Message` struct in `src/message.rs`:

```rust
pub struct Message {
    pub topic: String,
    pub payload: serde_json::Value,
}
```
